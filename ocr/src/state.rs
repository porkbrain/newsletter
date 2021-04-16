use crate::{prelude::*, vision::Ocr};
use shared::{S3Ext, SqsExt};

pub struct State {
    pub conf: Conf,
    pub sqs: Box<dyn SqsExt>,
    pub s3: Box<dyn S3Ext>,
    pub vision: Box<dyn Ocr>,
}
