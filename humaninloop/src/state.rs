use crate::{html::Template, prelude::*};
use shared::rusoto_s3::S3Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct State {
    pub template: Arc<Template>,
    pub s3: shared::rusoto_s3::S3Client,
}

impl State {
    pub fn new() -> Result<Self> {
        Ok(Self {
            s3: S3Client::new(Default::default()),
            template: Arc::new(Template::new()?),
        })
    }
}
