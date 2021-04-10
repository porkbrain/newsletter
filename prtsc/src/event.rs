//! Message published by S3 when a new object is created.
//!
//! [https://docs.aws.amazon.com/AmazonS3/latest/dev/notification-content-structure.html]

use {serde::Deserialize, std::str::FromStr};

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewS3Object {
    pub region: String,
    pub key: String,
    pub bucket: String,
}

impl FromStr for NewS3Object {
    type Err = Error;

    fn from_str(message_body: &str) -> Result<Self> {
        let records =
            serde_json::from_str::<NewS3ObjectEvent>(message_body)?.records;
        // there should always be one event for one new object
        if records.len() != 1 {
            return Err(Error::new(format!(
                "Unexpected number of records in event: {}",
                records.len(),
            ))
            .into());
        }
        // we've just checked that there's exactly one
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
