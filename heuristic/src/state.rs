use crate::prelude::*;
use shared::{http, S3Ext, SqsExt};

pub struct State {
    pub conf: Conf,
    pub sqs: Box<dyn SqsExt>,
    pub s3: Box<dyn S3Ext>,
    pub http_client: Box<dyn http::Client>,
}
