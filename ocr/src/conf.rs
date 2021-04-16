//! Name of each env var is the same as the property but in ALL_CAPS.

use {rusoto_core::Region, serde::Deserialize};

#[derive(Default, Deserialize, Debug)]
pub struct Conf {
    /// Where do we receive notifications about newly inserted html files?
    pub queue_url: String,
    /// All services _must_ run in the same region.
    pub region: Region,
    /// JSON like `{"installed":{"client_id": ... }}`
    pub gcp_secret: String,
    /// S3 where we store JSON files with OCR information.
    pub ocr_bucket: String,
}
