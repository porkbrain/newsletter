use crate::{html::Template, prelude::*};
use shared::rusoto_s3::S3Client;
use std::sync::{Arc, Mutex};
use tokio::fs;

#[derive(Clone)]
pub struct State {
    pub template: Arc<Template>,
    pub s3: shared::rusoto_s3::S3Client,
    pub newsletter_ids: Arc<Mutex<Vec<String>>>,
}

impl State {
    pub async fn new() -> Result<Self> {
        let newsletter_ids = {
            let l: Vec<_> = fs::read_to_string("humaninloop/.local/keys.txt")
                .await?
                .lines()
                .map(|s| s.to_owned())
                .collect();
            Arc::new(Mutex::new(l))
        };

        Ok(Self {
            newsletter_ids,
            s3: S3Client::new(Default::default()),
            template: Arc::new(Template::new()?),
        })
    }
}
