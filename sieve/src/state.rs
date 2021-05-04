use crate::prelude::*;
use shared::{S3Ext, SqsExt};
use sqlite::Connection;

pub struct State {
    pub conf: Conf,
    pub sqs: Box<dyn SqsExt>,
    pub s3: Box<dyn S3Ext>,
    pub db: Connection,
}
