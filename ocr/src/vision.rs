//! https://cloud.google.com/vision/quotas
//! https://cloud.google.com/vision/docs/supported-files

use crate::prelude::*;
use async_trait::async_trait;
use google_vision1::api::{
    AnnotateImageRequest, BatchAnnotateImagesRequest, Feature, Image,
    ImageSource,
};
use hyper_rustls::HttpsConnector;
use oauth2::{ServiceAccountAuthenticator, ServiceAccountKey};
use shared::vision::*;

pub use google_vision1::Vision;

#[async_trait]
pub trait Ocr {
    /// Given a URL for an image, it sends it to GCP Vision APIs and performs
    /// OCR annotation.
    async fn annotate(
        &self,
        image_url: String,
    ) -> Result<Option<Annotation>, Error>;
}

/// Creates a new Google client, ready to be used for querying the Vision APIs.
pub async fn new(gcp_secret: &str) -> Result<Vision, Error> {
    let secret: ServiceAccountKey = serde_json::from_str(gcp_secret)?;
    let auth = ServiceAccountAuthenticator::builder(secret).build().await?;

    let hub = Vision::new(
        hyper::Client::builder().build(HttpsConnector::with_native_roots()),
        auth,
    );

    Ok(hub)
}

#[async_trait]
impl Ocr for Vision {
    async fn annotate(
        &self,
        image_url: String,
    ) -> Result<Option<Annotation>, Error> {
        log::trace!("Getting annotation for image at {}", image_url);
        let annotate_req = BatchAnnotateImagesRequest {
            requests: Some(vec![AnnotateImageRequest {
                features: Some(vec![Feature {
                    type_: Some("TEXT_DETECTION".to_string()),
                    ..Default::default()
                }]),
                image: Some(Image {
                    source: Some(ImageSource {
                        image_uri: Some(image_url),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }]),
            ..Default::default()
        };

        let (_, data) = self.images().annotate(annotate_req).doit().await?;
        let annotation = data
            .responses
            .and_then(|mut r| r.pop()) // we only request one image
            .and_then(|r| {
                if let Some(error) = r.error {
                    log::error!("Got error from OCR APIs: {:#?}", error);
                }
                r.full_text_annotation
            })
            .ok_or_else(|| {
                Error::new("Empty response was returned from OCR")
            })?;

        Ok(Annotation::from(annotation))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    /// I define env with:
    ///
    /// ```bash
    /// export TEST_GCP_SECRET="$(cat .env.google.json)"
    /// ```
    #[ignore]
    #[tokio::test]
    async fn it_uses_gcp_ocr() {
        let gcp_secret = env::var("TEST_GCP_SECRET").unwrap();
        let vision = new(&gcp_secret).await.unwrap();
        let image_url =
        "https://upload.wikimedia.org/wikipedia/commons/d/d9/Plain_text.png";

        let annotation = vision.annotate(image_url.to_string()).await.unwrap();
        let _json = serde_json::to_string(&annotation).unwrap();
    }
}
