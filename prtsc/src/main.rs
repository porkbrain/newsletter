//! `prtsc` is a microservice which listens to SQS messages created by insertion
//! into an S3 bucket _IN_. _IN_ persists html files which are the contents of
//! received newsletter. It's setup in such a way that insertion pushes to SQS.
//!
//! `prtsc` [`handle`]s each message by capturing a screenshot and uploading it
//! to another S3 bucket _OUT_. _OUT_ persists the screenshot. The action of
//! insertion of a screenshot in the _OUT_ S3 publishes a new SQS message, which
//! is handled the next service in this pipeline.
//!
//! # Concurrency
//! `prtsc` handles at most one message. We poll the SQS with parameter which
//! amounts to `LIMIT 1`. Each message also contains a record about exactly one
//! newly inserted html file.
//!
//! This simplifies the logic tremendously. Instead of having several concurrent
//! threads in the microservice, we keep it single threaded and instead spawn
//! multiple replicas as necessary.
//!
//! # Gecko
//! To take screenshots, we rely on [geckodriver][gecko]. When this binary is
//! put into a container, the driver runs as a background process (along with
//! Xvfb for monitor simulation). While usually the container ethos is to have
//! one service per container, we are running 3 services in one container!
//!
//! The reason for this is error handling. I observed that sometimes the session
//! that runs in the driver breaks (starts returning errors), or that Xvfb
//! crashes. When something crashes, this binary will know about the error.
//! However, the containers running the driver etc don't restart when this
//! happens. Therefore, when everything is in one container and an error occurs,
//! we restart everything together.
//!
//! [gecko]: https://github.com/mozilla/geckodriver/releases

mod browser;
mod conf;
mod error;
mod prelude;
mod state;

use dotenv::dotenv;
use prelude::*;
use rusoto_s3::S3Client;
use rusoto_sqs::{Message, SqsClient};
use state::State;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    env_logger::init();
    log::info!("Starting prtsc v{}", env!("CARGO_PKG_VERSION"));

    let conf = envy::from_env::<Conf>()?;
    let sqs = Box::new(SqsClient::new(conf.region.clone()));
    let s3 = Box::new(S3Client::new(conf.region.clone()));
    let browser = Box::new(browser::connect(&conf.gecko_url).await?);
    let queue_url = conf.input_queue_url.clone();

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
/// 3. Stores the screenshot to an S3.
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
    log::trace!("Capturing a screenshot of html file at {}", url);
    let screenshot = state.browser.capture_jpeg_screenshot(&url).await?;
    if screenshot.len() > state.conf.max_screenshot_size {
        log::warn!(
            "Screenshot of {} is {} bytes, that's {} bytes too many",
            url,
            screenshot.len(),
            screenshot.len() - state.conf.max_screenshot_size
        );
    }

    // 3.
    log::trace!(
        "Captured screenshot of {} bytes, uploading to S3",
        screenshot.len()
    );
    state
        .s3
        .put(
            state.conf.screenshot_bucket_name.clone(),
            record.key,
            screenshot,
            shared::s3::PutConf {
                acl: Some("public-read".to_string()),
                cache_control: Some("public, immutable".to_string()),
                content_type: Some("image/jpeg".to_string()),
            },
        )
        .await?;

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
    use crate::browser::Headless;
    use async_trait::async_trait;
    use rusoto_core::Region;
    use shared::tests::*;

    #[tokio::test]
    async fn it_captures_screenshot_and_uploads_to_s3_and_deletes_message() {
        let receipt_handle = "test";
        let input_queue_url = "queue_url";
        let screenshot_bucket_name = "png_bucket";
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
            bucket: screenshot_bucket_name.to_string(),
            key: object_key.to_string(),
            body: body.clone(),
            conf: shared::s3::PutConf {
                acl: Some("public-read".to_string()),
                cache_control: Some("public, immutable".to_string()),
                content_type: Some("image/jpeg".to_string()),
            },
        };

        let sqs_stub = SqsStub {
            queue_url: input_queue_url.to_string(),
            receipt_handle: receipt_handle.to_string(),
        };

        let browser_stub = BrowserStub {
            url: format!(
                "https://s3-{}.amazonaws.com/{}/{}",
                region.name(),
                html_bucket,
                object_key
            ),
            screenshot: body.clone(),
        };

        let conf = Conf {
            max_screenshot_size: 20,
            screenshot_bucket_name: screenshot_bucket_name.to_string(),
            input_queue_url: input_queue_url.to_string(),
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
        screenshot: Vec<u8>,
    }

    #[async_trait]
    impl Headless for BrowserStub {
        async fn capture_jpeg_screenshot(
            &mut self,
            url: &str,
        ) -> Result<Vec<u8>, Error> {
            assert_eq!(url, &self.url);
            Ok(self.screenshot.clone())
        }
    }
}
