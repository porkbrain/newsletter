mod anchor;
mod conf;
mod db;
mod error;
mod prelude;
mod select;
mod state;

use dotenv::dotenv;
use prelude::*;
use shared::{
    anchor::Anchor,
    rusoto_sqs::{Message, SqsClient},
    vision::Annotation,
};
use shared::{rusoto_s3::S3Client, Phrases};
use sqlite::Connection;
use state::State;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    env_logger::init();
    log::info!("Starting sieve v{}", env!("CARGO_PKG_VERSION"));

    let conf = envy::from_env::<Conf>()?;
    let sqs = Box::new(SqsClient::new(conf.region.clone()));
    let s3 = Box::new(S3Client::new(conf.region.clone()));
    let db = Connection::open(&conf.database_path)?;
    let queue_url = conf.input_queue_url.clone();

    let mut state = State { conf, s3, sqs, db };

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
        .get(record.bucket.clone(), record.key.clone())
        .await?
        .ok_or_else(|| Error::new("OCR objects cannot have empty body"))?;
    let document: Phrases = serde_json::from_slice(&body)?;

    // 2.
    let (mut deals, mut vouchers) = select::deals_and_vouchers(document.inner());

    if deals.is_empty() && vouchers.is_empty() {
        log::debug!("There are no vouchers nor deals for {}", record.key);
    } else {
        let anchor_res = state
            .s3
            .get(state.conf.anchor_bucket_name.clone(), record.key.clone())
            .await;

        if let Ok(Some(anchors)) = anchor_res {
            let ocr = state
                .s3
                .get(state.conf.ocr_bucket_name.clone(), record.key.clone())
                .await?
                .ok_or_else(|| {
                    Error::new(format!("No OCR body for {}", record.key))
                })?;

            let ocr: Annotation = serde_json::from_slice(&ocr)?;
            let anchors: Vec<Anchor> = serde_json::from_slice(&anchors)?;

            anchor::find_hrefs_for_resources(
                anchors,
                ocr,
                &mut deals,
                &mut vouchers,
            );
        }

        db::insert(&state.db, &record.key, deals, vouchers)?;
    }

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
