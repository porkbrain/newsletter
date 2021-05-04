//! Name of each env var is the same as the property but in ALL_CAPS.

use {serde::Deserialize, shared::rusoto_core::Region};

#[derive(Default, Deserialize, Debug)]
pub struct Conf {
    /// Where do we receive notifications about newly inserted files?
    pub input_queue_url: String,
    /// All services _must_ run in the same region.
    ///
    /// # Default
    /// The [`Default` trait][default] results in rusoto reading the environment
    /// variable `AWS_DEFAULT_REGION`.
    ///
    /// [default]: https://docs.rs/rusoto_core/0.46.0/rusoto_core/enum.Region.html#default
    #[serde(default = "Default::default")]
    pub region: Region,
    /// JSON like `{"installed":{"client_id": ... }}`
    pub gcp_secret: String,
    /// S3 where we store JSON files with OCR information.
    pub ocr_bucket_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn it_creates_conf_from_env() {
        env::set_var("INPUT_QUEUE_URL", "queuetest");
        env::set_var("OCR_BUCKET_NAME", "buckettest");
        env::set_var("GCP_SECRET", "gcptest");
        env::set_var("AWS_DEFAULT_REGION", "eu-west-1");

        let conf = envy::from_env::<Conf>().unwrap();

        assert_eq!(conf.input_queue_url, "queuetest");
        assert_eq!(conf.ocr_bucket_name, "buckettest");
        assert_eq!(conf.gcp_secret, "gcptest");
        assert_eq!(conf.region, Region::EuWest1);
    }
}
