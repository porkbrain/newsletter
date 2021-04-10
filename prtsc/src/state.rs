use {
    async_trait::async_trait,
    rusoto_s3::{PutObjectRequest, S3Client, S3},
    rusoto_sqs::{
        DeleteMessageRequest, Message, ReceiveMessageRequest, Sqs, SqsClient,
    },
};

use crate::{browser, prelude::*};

pub struct State {
    pub conf: Conf,
    pub browser: Box<dyn browser::Headless>,
    pub s3: Box<dyn S3Ext>,
    pub sqs: Box<dyn SqsExt>,
}

/// Implements only methods which this project requires instead of all
/// [`rusoto_sqs::Sqs`] methods, which makes it more comfortable to write stubs
/// and test it.
#[async_trait]
pub trait SqsExt {
    async fn receive(&self, queue_url: String) -> Result<Option<Message>>;
    async fn delete(
        &self,
        queue_url: String,
        receipt_handle: String,
    ) -> Result<()>;
}

#[async_trait]
impl SqsExt for SqsClient {
    async fn receive(&self, queue_url: String) -> Result<Option<Message>> {
        let req = ReceiveMessageRequest {
            queue_url,
            max_number_of_messages: Some(1), // simplifies logic, see docs
            wait_time_seconds: Some(20),     // max
            ..Default::default()
        };

        Ok(self
            .receive_message(req)
            .await?
            .messages
            .and_then(|messages| {
                debug_assert!(messages.len() <= 1);
                messages.into_iter().next()
            }))
    }

    async fn delete(
        &self,
        queue_url: String,
        receipt_handle: String,
    ) -> Result<()> {
        let req = DeleteMessageRequest {
            receipt_handle,
            queue_url,
        };
        self.delete_message(req).await?;
        Ok(())
    }
}

/// Implements only methods which this project requires instead of all
/// [`rusoto_s3::S3`] methods, which makes it more comfortable to write stubs
/// and test it.
#[async_trait]
pub trait S3Ext {
    async fn put(
        &self,
        bucket: String,
        key: String,
        body: Vec<u8>,
    ) -> Result<()>;
}

#[async_trait]
impl S3Ext for S3Client {
    async fn put(
        &self,
        bucket: String,
        key: String,
        body: Vec<u8>,
    ) -> Result<()> {
        let req = PutObjectRequest {
            acl: Some("public-read".to_string()),
            body: Some(body.into()),
            bucket,
            cache_control: Some("public, immutable".to_string()),
            content_type: Some("image/png".to_string()),
            key,
            storage_class: Some("REDUCED_REDUNDANCY".to_string()),
            ..Default::default()
        };
        self.put_object(req).await?;
        Ok(())
    }
}
