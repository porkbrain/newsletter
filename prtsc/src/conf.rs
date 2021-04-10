//! Name of each env var is the same as the property but in ALL_CAPS.

use {rusoto_core::Region, serde::Deserialize};

#[derive(Default, Deserialize, Debug)]
pub struct Conf {
    /// Where do we receive notifications about new html files added?
    pub queue_url: String,
    /// On what address does the proxy to the headless browser sit?
    /// TODO: more docs
    pub gecko_url: String,
    /// All services _must_ run in the same region.
    pub region: Region,
    /// After taking a screenshot, where should we persist it?
    pub png_bucket: String,
    /// Png which has more bytes that this many is ignored.
    #[serde(default = "default_max_png_size")]
    pub max_png_size: usize,
}

fn default_max_png_size() -> usize {
    2_500_000 // 2.5MB
}
