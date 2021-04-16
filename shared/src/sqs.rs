use {
    async_trait::async_trait,
    rusoto_core::RusotoError,
    rusoto_sqs::{
        DeleteMessageError, DeleteMessageRequest, GetQueueAttributesError,
        GetQueueAttributesRequest, Message, ReceiveMessageError,
        ReceiveMessageRequest, Sqs, SqsClient,
    },
    std::{collections::HashMap, time::Duration},
};

/// Implements only methods which this project requires instead of all
/// [`rusoto_sqs::Sqs`] methods, which makes it more comfortable to write stubs
/// and test it.
#[async_trait]
pub trait SqsExt {
    async fn receive_one(
        &self,
        queue_url: String,
    ) -> Result<Option<Message>, RusotoError<ReceiveMessageError>>;

    async fn delete(
        &self,
        queue_url: String,
        receipt_handle: String,
    ) -> Result<(), RusotoError<DeleteMessageError>>;

    async fn get_attributes(
        &self,
        queue_url: String,
        attrs: Vec<String>,
    ) -> Result<HashMap<String, String>, RusotoError<GetQueueAttributesError>>;
}

/// The brick and bones of any SQS listening microservice. Keeps on receiving
/// messages from an sqs in a loop. On an unrecoverable error, stops everything
/// and returns it.
#[async_trait]
impl SqsExt for SqsClient {
    async fn receive_one(
        &self,
        queue_url: String,
    ) -> Result<Option<Message>, RusotoError<ReceiveMessageError>> {
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
    ) -> Result<(), RusotoError<DeleteMessageError>> {
        let req = DeleteMessageRequest {
            receipt_handle,
            queue_url,
        };
        self.delete_message(req).await?;
        Ok(())
    }

    async fn get_attributes(
        &self,
        queue_url: String,
        attrs: Vec<String>,
    ) -> Result<HashMap<String, String>, RusotoError<GetQueueAttributesError>>
    {
        let req = GetQueueAttributesRequest {
            queue_url,
            attribute_names: Some(attrs),
        };
        let res = self.get_queue_attributes(req).await?.attributes;
        Ok(res.unwrap_or_default())
    }
}

/// Returns the visibility timeout of the input SQS. That is, returns the
/// duration of how long a message is "invisible" to other consumers after it's
/// received.
pub async fn get_visibility_timeout(
    sqs: impl SqsExt,
    queue_url: String,
) -> Result<Option<Duration>, RusotoError<GetQueueAttributesError>> {
    log::info!("Requesting visibility timeout from SQS {}", queue_url);
    let attrs = sqs
        .get_attributes(queue_url, vec!["VisibilityTimeout".to_string()])
        .await?;
    log::trace!("Queue attrs: {:#?}", attrs);
    attrs
        .get("VisibilityTimeout")
        .and_then(|s| s.parse::<u64>().ok())
        .map(Duration::from_secs)
        .map(Ok)
        .transpose()
}
