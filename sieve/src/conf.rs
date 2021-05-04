//! Name of each env var is the same as the property but in ALL_CAPS.

use serde::Deserialize;
use shared::rusoto_core::Region;

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
    /// S3 where we store JSON files with link positions and destinations.
    pub anchor_bucket_name: String,
    /// Where we store OCR positions of strings in email.
    pub ocr_bucket_name: String,
    /// Path to the sqlite3 file into which we store results.
    pub database_path: String,
}
