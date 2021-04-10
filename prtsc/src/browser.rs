//! TODO: Document connections to the browser.

use async_trait::async_trait;

use crate::prelude::*;

pub async fn connect(gecko_url: &str) -> Result<fantoccini::Client> {
    fantoccini::ClientBuilder::rustls()
        .connect(gecko_url)
        .await
        .map_err(|e| {
            log::error!("Cannot start gecko client: {}", e);
            Error::fatal(e)
        })
}

#[async_trait]
pub trait Headless {
    async fn capture_png_screenshot(&mut self, url: &str) -> Result<Vec<u8>>;
}

#[async_trait]
impl Headless for fantoccini::Client {
    async fn capture_png_screenshot(&mut self, url: &str) -> Result<Vec<u8>> {
        self.goto(url).await?;
        let png = self.screenshot().await?;
        Ok(png)
    }
}
