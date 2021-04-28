pub mod s3;
pub mod sqs;
pub mod vision;

pub use rusoto_core;
pub use rusoto_s3;
pub use rusoto_sqs;

pub use {s3::S3Ext, sqs::SqsExt};

#[cfg(feature = "test_utils")]
pub mod tests;
