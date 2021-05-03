//! `ocr` is a microservice which listens to SQS messages created by insertions
//! into an S3 bucket _IN_. _IN_ persists png screenshot of each newsletter.
//!
//! Screenshots are then sent to [Google's Vision API][vision-api] for text
//! detection. The output from the text detection is trimmed of unnecessary
//! information and stored as a JSON file in _OUT_ S3 bucket. The advantage of
//! not storing the parsed OCR in a database is that S3 allows us to create SQS
//! notifications on insertion, and therefore follow the same design pattern in
//! many services.  Also running OCR is quite expensive and storage in S3 is
//! cheaper and more reliable.
//!
//! # Batching
//! Following proposal is not yet implemented because the expenses on OCR are
//! not worth the effort. [Google Vision APIs][vision-api-pricing] costs $0.0015
//! per image scanned.
//!
//! Because GCP Vision API pricing is per image, we cut costs by stitching as
//! many screenshots as possible into one image.
//!
//! Since the resulting API gives us bounding box information, we can determine
//! which OCR'd text belongs to which screenshot.
//!
//! We keep on receiving messages from the SQS until
//! a) we reach the size limit of Vision API;
//! b) the oldest message we keep is reaching the end of its visibility timeout.
//!
//! We can calculate the approximate memory usage by considering how much data
//! is necessary to reach a).
//!
//! [vision-api-pricing]: https://cloud.google.com/vision/pricing
//! [vision-api]: https://cloud.google.com/vision/docs/ocr

mod conf;
mod error;
mod prelude;
mod state;
mod vision;

use dotenv::dotenv;
use prelude::*;
use shared::rusoto_s3::S3Client;
use shared::rusoto_sqs::{Message, SqsClient};
use state::State;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    env_logger::init();
    log::info!("Starting ocr v{}", env!("CARGO_PKG_VERSION"));

    let conf = envy::from_env::<Conf>()?;
    let sqs = Box::new(SqsClient::new(conf.region.clone()));
    let s3 = Box::new(S3Client::new(conf.region.clone()));
    let vision = Box::new(vision::new(&conf.gcp_secret).await?);
    let queue_url = conf.input_queue_url.clone();

    let mut state = State {
        conf,
        s3,
        sqs,
        vision,
    };

    // we assume something is supervising this service
    loop {
        log::trace!("Waiting for a new message");
        if let Some(message) =
            state.sqs.as_ref().receive_one(queue_url.clone()).await?
        {
            match handle(&mut state, message).await {
                Ok(_) => (),
                Err(e) if e.is_recoverable() => {
                    log::error!("Cannot process message: {}", e);
                }
                Err(e) => {
                    log::error!("Fatal error: {}", e);
                    return Err(e);
                }
            }
        }
    }
}

/// 1. Extracts information about newly inserted object, focus on its url.
///
/// 2. Runs an OCR job with Vision API and strips unnecessary data from the
///    response.
///
/// 3. Stores the output of the OCR job in a dedicated S3.
///
/// 4. Deletes the incoming message to mark the task as "done".
async fn handle(state: &mut State, message: Message) -> Result<(), Error> {
    let Message {
        body,
        receipt_handle,
        message_id,
        ..
    } = message;
    let receipt_handle = receipt_handle
        .ok_or_else(|| Error::new("Each message must have a receipt handle"))?;
    let body = body.ok_or_else(|| {
        Error::new(format!(
            "Received message {:?} with an empty body",
            message_id
        ))
    })?;

    // 1.
    log::trace!("Received a new message with body: \n\n{}", body);
    let record = shared::s3::NewS3Object::from_str(&body)?;
    let url = format!(
        "https://s3-{}.amazonaws.com/{}/{}",
        record.region, record.bucket, record.key
    );

    // 2.
    if let Some(annotation) = state.vision.annotate(url).await? {
        let json = serde_json::to_string(&annotation)?;
        // 3.
        log::trace!("Saving OCr annotation for {} into s3", record.key);
        state
            .s3
            .put(
                state.conf.ocr_bucket_name.clone(),
                record.key,
                json.into(),
                shared::s3::PutConf {
                    content_type: Some("application/json".to_string()),
                    ..Default::default()
                },
            )
            .await?;
    } else {
        log::warn!("No text found in image {}", record.key);
    }

    // 4.
    log::trace!(
        "Deleting message {:?} (handle {:?})",
        message_id,
        receipt_handle
    );
    state
        .sqs
        .delete(state.conf.input_queue_url.clone(), receipt_handle)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        state::State,
        vision::{Ocr},
    };
    use shared::vision::Annotation;
    use async_trait::async_trait;
    use shared::rusoto_core::Region;
    use shared::tests::*;

    #[tokio::test]
    async fn it_ocrs_and_uploads_to_s3_and_deletes_message() {
        let receipt_handle = "test";
        let input_queue_url = "queue_url";
        let png_bucket = "png_bucket";
        let ocr_bucket_name = "ocr_bucket";
        let object_key = "test_key";
        let body: Vec<u8> = serde_json::to_string(&Annotation::default())
            .unwrap()
            .into();
        let region = Region::EuWest2;

        let message = Message {
            // https://docs.aws.amazon.com/AmazonS3/latest/userguide/notification-content-structure.html
            body: Some(
                serde_json::to_string(&serde_json::json!({
                    "Records": [
                       {
                          "awsRegion": "eu-west-2",
                          "s3": {
                             "bucket": {
                                "name": png_bucket,
                             },
                             "object": {
                                "key": object_key,
                             }
                          }
                       }
                    ]
                }))
                .unwrap(),
            ),
            receipt_handle: Some(receipt_handle.to_string()),
            ..Default::default()
        };

        let s3_stub = S3Stub {
            bucket: ocr_bucket_name.to_string(),
            key: object_key.to_string(),
            body: body.clone(),
            conf: shared::s3::PutConf {
                content_type: Some("application/json".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };

        let sqs_stub = SqsStub {
            queue_url: input_queue_url.to_string(),
            receipt_handle: receipt_handle.to_string(),
        };

        let vision_stub = VisionStub {
            annotation: Default::default(),
            image_url: format!(
                "https://s3-{}.amazonaws.com/{}/{}",
                region.name(),
                png_bucket,
                object_key
            ),
        };

        let conf = Conf {
            ocr_bucket_name: ocr_bucket_name.to_string(),
            input_queue_url: input_queue_url.to_string(),
            region,
            ..Default::default()
        };

        let mut state = State {
            conf,
            s3: Box::new(s3_stub),
            sqs: Box::new(sqs_stub),
            vision: Box::new(vision_stub),
        };

        handle(&mut state, message).await.unwrap();
    }

    struct VisionStub {
        image_url: String,
        annotation: Annotation,
    }

    #[async_trait]
    impl Ocr for VisionStub {
        async fn annotate(
            &self,
            image_url: String,
        ) -> Result<Option<Annotation>, Error> {
            assert_eq!(self.image_url, image_url);
            Ok(Some(self.annotation.clone()))
        }
    }
}
