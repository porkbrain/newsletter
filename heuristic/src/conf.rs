//! Name of each env var is the same as the property but in ALL_CAPS.

use {serde::Deserialize, shared::rusoto_core::Region};

#[derive(Default, Deserialize, Debug)]
pub struct Conf {
    /// Where do we receive notifications about newly inserted html files?
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
    /// On what URL can we reach dealc to categorize phrases.
    pub dealc_url: String,
    /// On what URL can we reach voucherc to categorize vouchers.
    pub voucherc_url: String,
    /// API key for OpenAI account.
    pub openai_key: String,
    /// Where can we reach OpenAI servers.
    pub openai_completion_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn it_creates_conf_from_env() {
        env::set_var("INPUT_QUEUE_URL", "queuetest");
        env::set_var("DEALC_URL", "dealcurl");
        env::set_var("VOUCHERC_URL", "vouchercurl");
        env::set_var("AWS_DEFAULT_REGION", "eu-west-1");
        env::set_var("OPENAI_KEY", "abckey");
        env::set_var("OPENAI_COMPLETION_URL", "url");

        let conf = envy::from_env::<Conf>().unwrap();

        assert_eq!(conf.input_queue_url, "queuetest");
        assert_eq!(conf.region, Region::EuWest1);
        assert_eq!(conf.dealc_url, "dealcurl");
        assert_eq!(conf.voucherc_url, "vouchercurl");
        assert_eq!(conf.openai_key, "abckey");
        assert_eq!(conf.openai_completion_url, "url");
    }
}
