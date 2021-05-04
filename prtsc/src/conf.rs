//! Name of each env var is the same as the property but in ALL_CAPS.

use serde::Deserialize;
use shared::rusoto_core::Region;

#[derive(Default, Deserialize, Debug)]
pub struct Conf {
    /// Where do we receive notifications about new html files added?
    pub input_queue_url: String,
    /// On what address does the proxy to the headless browser sit?
    /// This is set by the Dockerfile.
    pub gecko_url: String,
    /// All services _must_ run in the same region.
    ///
    /// # Default
    /// The [`Default` trait][default] results in rusoto reading the environment
    /// variable `AWS_DEFAULT_REGION`.
    ///
    /// [default]: https://docs.rs/rusoto_core/0.46.0/rusoto_core/enum.Region.html#default
    #[serde(default = "Default::default")]
    pub region: Region,
    /// After taking a screenshot, where should we persist it?
    pub screenshot_bucket_name: String,
    /// Png which has more bytes that this many is ignored.
    #[serde(default = "default_max_screenshot_size")]
    pub max_screenshot_size: usize,
    /// We read all links and their bounding boxes, so that we can cross
    /// reference them later with found deals and vouchers. This is the name of
    /// the bucket where we store those links.
    pub anchor_bucket_name: String,
}

fn default_max_screenshot_size() -> usize {
    5_000_000 // 5MB
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn it_creates_conf_from_env() {
        env::set_var("INPUT_QUEUE_URL", "queuetest");
        env::set_var("SCREENSHOT_BUCKET_NAME", "buckettest");
        env::set_var("GECKO_URL", "geckotest");
        env::set_var("AWS_DEFAULT_REGION", "eu-west-1");
        env::set_var("ANCHOR_BUCKET_NAME", "anchortest");

        let conf = envy::from_env::<Conf>().unwrap();

        assert_eq!(conf.input_queue_url, "queuetest");
        assert_eq!(conf.screenshot_bucket_name, "buckettest");
        assert_eq!(conf.anchor_bucket_name, "anchortest");
        assert_eq!(conf.gecko_url, "geckotest");
        assert_eq!(conf.region, Region::EuWest1);
    }
}
