use crate::prelude::*;
use async_trait::async_trait;
use image::{io::Reader as ImageReader, ImageFormat, ImageOutputFormat};
use shared::anchor::Anchor;
use std::io::Cursor;

const SELECT_ANCHORS_SCRIPT_JS: &str = r#"
    const anchors = Array.from(document.querySelectorAll("a"));

    return anchors
        .map((a) => {
          const href = a.getAttribute("href");
          return {
            href,
            top: Math.round(a.offsetTop),
            left: Math.round(a.offsetLeft),
            width: Math.round(a.offsetWidth),
            height: Math.round(a.offsetHeight),
          };
        })
        .filter(
          ({ href }) =>
            href && href.startsWith("http") && !href.includes("unsubscribe")
        );
"#;

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
    async fn capture_jpeg_screenshot_and_extract_anchors(
        &mut self,
        url: &str,
    ) -> Result<(Vec<u8>, Vec<Anchor>), Error>;
}

#[async_trait]
impl Headless for fantoccini::Client {
    async fn capture_jpeg_screenshot_and_extract_anchors(
        &mut self,
        url: &str,
    ) -> Result<(Vec<u8>, Vec<Anchor>), Error> {
        self.goto(url).await?;
        log::trace!("Navigated to {}, taking screenshot now", url);

        let anchors: Vec<Anchor> = serde_json::from_value(
            self.execute(SELECT_ANCHORS_SCRIPT_JS, vec![]).await?,
        )?;

        let png = self.screenshot().await?;
        let img = ImageReader::with_format(Cursor::new(png), ImageFormat::Png)
            .decode()?;
        let mut jpeg: Vec<u8> = Vec::new();
        img.write_to(&mut jpeg, ImageOutputFormat::Jpeg(90))?;
        Ok((jpeg, anchors))
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

        let (_jpeg, _anchors) = client
            .capture_jpeg_screenshot_and_extract_anchors(html_url)
            .await
            .unwrap();

        client.close().await.unwrap();
    }
}
