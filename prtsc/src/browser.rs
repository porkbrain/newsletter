use crate::prelude::*;
use async_trait::async_trait;
use image::{io::Reader as ImageReader, ImageFormat, ImageOutputFormat};
use std::io::Cursor;

pub async fn connect(gecko_url: &str) -> Result<fantoccini::Client, Error> {
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
    async fn capture_jpeg_screenshot(
        &mut self,
        url: &str,
    ) -> Result<Vec<u8>, Error>;
}

#[async_trait]
impl Headless for fantoccini::Client {
    async fn capture_jpeg_screenshot(
        &mut self,
        url: &str,
    ) -> Result<Vec<u8>, Error> {
        self.goto(url).await?;
        log::trace!("Navigated to {}, taking screenshot now", url);
        let png = self.screenshot().await?;
        let img = ImageReader::with_format(Cursor::new(png), ImageFormat::Png)
            .decode()?;
        let mut jpeg: Vec<u8> = Vec::new();
        img.write_to(&mut jpeg, ImageOutputFormat::Jpeg(90))?;
        Ok(jpeg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    // To test screenshot capture, I like to use
    // https://github.com/cv-library/docker-geckodriver and expose the 4444 port
    // on localhost.
    #[ignore]
    #[tokio::test]
    async fn it_should_capture_fullscreen() {
        let html_url = "https://seznam.cz";

        let driver_url = env::var("TEST_GECKO_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:4444".to_string());
        let mut client = connect(&driver_url).await.unwrap();

        let _jpeg = client.capture_jpeg_screenshot(html_url).await.unwrap();
    }
}
