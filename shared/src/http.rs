use async_trait::async_trait;

#[async_trait]
pub trait Client {
    async fn post_json(
        &self,
        url: &str,
        body: &serde_json::Value,
    ) -> Result<Vec<u8>, reqwest::Error>;
}

#[async_trait]
impl Client for reqwest::Client {
    async fn post_json(
        &self,
        url: &str,
        body: &serde_json::Value,
    ) -> Result<Vec<u8>, reqwest::Error> {
        let response =
            self.post(url).json(body).send().await?.error_for_status()?;

        Ok(response.bytes().await?.to_vec())
    }
}
