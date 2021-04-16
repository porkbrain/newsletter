use {
    crate::{browser, prelude::*},
    shared::{S3Ext, SqsExt},
};

pub struct State {
    pub conf: Conf,
    pub browser: Box<dyn browser::Headless>,
    pub s3: Box<dyn S3Ext>,
    pub sqs: Box<dyn SqsExt>,
}
