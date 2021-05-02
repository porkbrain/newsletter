use super::{S3Ext, SqsExt};
use async_trait::async_trait;
use rusoto_core::RusotoError;
use rusoto_s3::{GetObjectError, PutObjectError};
use rusoto_sqs::{
    DeleteMessageError, GetQueueAttributesError, Message, ReceiveMessageError,
};
use serde::de::DeserializeOwned;
use std::{collections::HashMap, io};

#[derive(Default)]
pub struct S3Stub {
    pub bucket: String,
    pub key: String,
    pub body: Vec<u8>,
    pub conf: crate::s3::PutConf,
    pub object_json: serde_json::Value,
}

#[derive(Default)]
pub struct SqsStub {
    pub queue_url: String,
    pub receipt_handle: String,
}

#[async_trait]
impl S3Ext for S3Stub {
    async fn put(
        &self,
        bucket: String,
        key: String,
        body: Vec<u8>,
        conf: crate::s3::PutConf,
    ) -> Result<(), RusotoError<PutObjectError>> {
        assert_eq!(bucket, self.bucket);
        assert_eq!(key, self.key);
        assert_eq!(body, self.body);
        assert_eq!(conf, self.conf);
        Ok(())
    }

    async fn get<T, E>(
        &self,
        bucket: String,
        key: String,
    ) -> Result<Option<T>, E>
    where
        T: DeserializeOwned,
        E: From<RusotoError<GetObjectError>>
            + From<serde_json::Error>
            + From<io::Error>,
    {
        assert_eq!(bucket, self.bucket);
        assert_eq!(key, self.key);
        Ok(serde_json::from_value(self.object_json.clone()).unwrap())
    }
}

#[async_trait]
impl SqsExt for SqsStub {
    async fn receive_one(
        &self,
        _: String,
    ) -> Result<Option<Message>, RusotoError<ReceiveMessageError>> {
        unimplemented!()
    }

    async fn delete(
        &self,
        queue_url: String,
        receipt_handle: String,
    ) -> Result<(), RusotoError<DeleteMessageError>> {
        assert_eq!(queue_url, self.queue_url);
        assert_eq!(receipt_handle, self.receipt_handle);
        Ok(())
    }

    async fn get_attributes(
        &self,
        _queue_url: String,
        _attrs: Vec<String>,
    ) -> Result<HashMap<String, String>, RusotoError<GetQueueAttributesError>>
    {
        unimplemented!()
    }
}
