mod conf;
mod error;
mod models;
mod parse;
mod prelude;
mod sources;
mod state;

use dotenv::dotenv;
use prelude::*;
use shared::{
    reqwest::{self, header},
    rusoto_s3::S3Client,
    rusoto_sqs::{Message, SqsClient},
    vision::Annotation,
};
use state::State;
use std::str::FromStr;

use crate::sources::get_phrases_with_estimates;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    env_logger::init();
    log::info!("Starting ocr v{}", env!("CARGO_PKG_VERSION"));

    let conf = envy::from_env::<Conf>()?;
    let sqs = Box::new(SqsClient::new(conf.region.clone()));
    let s3 = Box::new(S3Client::new(conf.region.clone()));
    let http_client = Box::new({
        let mut headers = header::HeaderMap::new();
        let mut auth_value = header::HeaderValue::from_str(&format!(
            "Bearer {}",
            conf.openai_key
        ))
        .expect("Invalid openai key characters");
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);

        reqwest::Client::builder()
            .default_headers(headers)
            .build()?
    });
    let queue_url = conf.input_queue_url.clone();

    let mut state = State {
        conf,
        s3,
        sqs,
        http_client,
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

/// 1. Load OCR output from S3 bucket.
///
/// 2. Use various methods to predict what are vouchers and what are deals.
///
///
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
    let body = state
        .s3
        .get(record.bucket, record.key)
        .await?
        .ok_or_else(|| Error::new("OCR objects cannot have empty body"))?;
    let annotation: Annotation = serde_json::from_slice(&body)?;

    // 2.
    let document = get_phrases_with_estimates(
        &state.conf,
        state.http_client.as_ref(),
        &annotation.text,
    )
    .await?;

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
    //
}
