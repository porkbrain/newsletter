use async_trait::async_trait;
use futures::{StreamExt, TryStreamExt};
use rusoto_core::RusotoError;
use rusoto_s3::{
    GetObjectError, GetObjectRequest, PutObjectError, PutObjectRequest,
    S3Client, S3,
};
use serde::Deserialize;
use std::str::FromStr;

/// Implements only methods which this project requires instead of all
/// [`rusoto_s3::S3`] methods, which makes it more comfortable to write stubs
/// and test it.
#[async_trait]
pub trait S3Ext {
    async fn put(
        &self,
        bucket: String,
        key: String,
        body: Vec<u8>,
        conf: PutConf,
    ) -> Result<(), RusotoError<PutObjectError>>;

    async fn get(
        &self,
        bucket: String,
        key: String,
    ) -> Result<Option<Vec<u8>>, RusotoError<GetObjectError>>;
}

#[derive(Default, PartialEq, Debug)]
pub struct PutConf {
    pub acl: Option<String>,
    pub cache_control: Option<String>,
    pub content_type: Option<String>,
}

#[async_trait]
impl S3Ext for S3Client {
    async fn put(
        &self,
        bucket: String,
        key: String,
        body: Vec<u8>,
        conf: PutConf,
    ) -> Result<(), RusotoError<PutObjectError>> {
        let req = PutObjectRequest {
            acl: conf.acl,
            cache_control: conf.cache_control,
            content_type: conf.content_type,
            body: Some(body.into()),
            bucket,
            key,
            storage_class: Some("REDUCED_REDUNDANCY".to_string()),
            ..Default::default()
        };
        self.put_object(req).await?;
        Ok(())
    }

    async fn get(
        &self,
        bucket: String,
        key: String,
    ) -> Result<Option<Vec<u8>>, RusotoError<GetObjectError>> {
        let body = self
            .get_object(GetObjectRequest {
                bucket,
                key,
                ..Default::default()
            })
            .await?
            .body;

        if let Some(body) = body {
            let body: Vec<_> = body
                .into_stream()
                .collect::<Vec<_>>()
                .await
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect();

            Ok(Some(body))
        } else {
            Ok(None)
        }
    }
}

/// Message published by S3 when a new object is created.
///
/// [https://docs.aws.amazon.com/AmazonS3/latest/dev/notification-content-structure.html]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewS3Object {
    pub region: String,
    pub key: String,
    pub bucket: String,
}

impl FromStr for NewS3Object {
    type Err = serde_json::Error;

    fn from_str(message_body: &str) -> Result<Self, Self::Err> {
        let records =
            serde_json::from_str::<NewS3ObjectEvent>(message_body)?.records;

        assert_eq!(
            1,
            records.len(),
            "There must always be exactly one record in an SQS message"
        );
        let record = records.into_iter().next().unwrap();

        Ok(Self {
            region: record.aws_region,
            key: record.s3.object.key,
            bucket: record.s3.bucket.name,
        })
    }
}

#[derive(Deserialize)]
struct NewS3ObjectEvent {
    #[serde(rename = "Records")]
    records: Vec<NewS3ObjectEventRecord>,
}

#[derive(Deserialize)]
struct NewS3ObjectEventRecord {
    #[serde(rename = "awsRegion")]
    aws_region: String,
    s3: NewS3ObjectEventRecordS3,
}

#[derive(Deserialize)]
struct NewS3ObjectEventRecordS3 {
    bucket: NewS3ObjectEventRecordS3Bucket,
    object: NewS3ObjectEventRecordS3Object,
}

#[derive(Deserialize)]
struct NewS3ObjectEventRecordS3Bucket {
    name: String,
}

#[derive(Deserialize)]
struct NewS3ObjectEventRecordS3Object {
    key: String,
}
