//! `prtsc` is a microservice which listens to SQS messages created by insertion
//! into an S3 bucket _IN_. _IN_ persists html files which are the contents of
//! received newsletter. It's setup in such a way that insertion pushes to SQS.
//!
//! `prtsc` [`handle`]s each message by capturing a png screenshot and uploading
//! it to another S3 bucket _OUT_. _OUT_ persists the screenshot.
//!
//! # Concurrency
//! `prtsc` handles at most one message. We poll the SQS with parameter which
//! amounts to `LIMIT 1`. Each message also contains a record about exactly one
//! newly inserted html file.
//!
//! This simplifies the logic tremendously. Instead of having several concurent
//! threads in the microservice, we keep it single threaded and instead spawn
//! multiple replicas as necessary.

mod browser;
mod conf;
mod error;
mod prelude;
mod state;

use {
    dotenv::dotenv,
    rusoto_s3::S3Client,
    rusoto_sqs::{Message, SqsClient},
    std::str::FromStr,
};

use {prelude::*, state::State};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    env_logger::init();

    let conf = envy::from_env::<Conf>()?;
    let sqs = Box::new(SqsClient::new(conf.region.clone()));
    let s3 = Box::new(S3Client::new(conf.region.clone()));
    let browser = Box::new(browser::connect(&conf.gecko_url).await?);
    let queue_url = conf.queue_url.clone();

    let mut state = State {
        browser,
        conf,
        sqs,
        s3,
    };

    // keeps polling new messages and exits if error occurs in
    // 1. connection to the sqs
    // 2. connection to the headless browser
    // that's why this service needs supervision
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

/// 1. Extracts the name of the object from the message and constructs the url
///    at which this object is reachable.
///
/// 2. Takes a screenshot of the object (expecting a html page).
///
/// 3. Stores the png screenshot to an S3.
///
/// 4. Deletes the message from SQS.
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
    log::trace!("Capturing a png of html file at {}", url);
    let png = state.browser.capture_png_screenshot(&url).await?;
    if png.len() > state.conf.max_png_size {
        log::warn!(
            "Screenshot of {} is {} bytes, that's {} bytes too many",
            url,
            png.len(),
            png.len() - state.conf.max_png_size
        );
    }

    // 3.
    log::trace!(
        "Captured screenshot of {} bytes, uploading to S3",
        png.len()
    );
    state
        .s3
        .put(state.conf.png_bucket.clone(), record.key, png)
        .await?;

    // 4.
    log::trace!(
        "Deleting message {:?} (handle {:?})",
        message_id,
        receipt_handle
    );
    state
        .sqs
        .delete(state.conf.queue_url.clone(), receipt_handle)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::browser::Headless;
    use async_trait::async_trait;
    use rusoto_core::Region;
    use shared::tests::*;

    #[tokio::test]
    async fn it_captures_screenshot_and_uploads_to_s3_and_deletes_message() {
        let receipt_handle = "test";
        let queue_url = "queue_url";
        let png_bucket = "png_bucket";
        let html_bucket = "html_bucket";
        let object_key = "test_key";
        let body = vec![0, 1, 2, 3, 4, 5, 6, 7];
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
                            "name": html_bucket,
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
            bucket: png_bucket.to_string(),
            key: object_key.to_string(),
            body: body.clone(),
        };

        let sqs_stub = SqsStub {
            queue_url: queue_url.to_string(),
            receipt_handle: receipt_handle.to_string(),
        };

        let browser_stub = BrowserStub {
            url: format!(
                "https://s3-{}.amazonaws.com/{}/{}",
                region.name(),
                html_bucket,
                object_key
            ),
            png: body.clone(),
        };

        let conf = Conf {
            max_png_size: 20,
            png_bucket: png_bucket.to_string(),
            queue_url: queue_url.to_string(),
            region,
            ..Default::default()
        };

        let mut state = State {
            conf,
            s3: Box::new(s3_stub),
            sqs: Box::new(sqs_stub),
            browser: Box::new(browser_stub),
        };

        handle(&mut state, message).await.unwrap();
    }

    struct BrowserStub {
        url: String,
        png: Vec<u8>,
    }

    #[async_trait]
    impl Headless for BrowserStub {
        async fn capture_png_screenshot(
            &mut self,
            url: &str,
        ) -> Result<Vec<u8>, Error> {
            assert_eq!(url, &self.url);
            Ok(self.png.clone())
        }
    }
}
