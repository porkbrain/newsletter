//! https://cloud.google.com/vision/quotas
//! https://cloud.google.com/vision/docs/supported-files

use crate::prelude::*;
use async_trait::async_trait;
use google_vision1::api::{
    AnnotateImageRequest, BatchAnnotateImagesRequest, Feature, Image,
    ImageSource, TextAnnotation as GAnnotation, Word as GWord,
};
use hyper_rustls::HttpsConnector;
use oauth2::{ServiceAccountAuthenticator, ServiceAccountKey};
use serde::Serialize;

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
            .and_then(|r| r.full_text_annotation)
            .ok_or_else(|| {
                Error::new("Empty response was returned from OCR")
            })?;

        Ok(Annotation::from(annotation))
    }
}

#[derive(Serialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Default, Clone))]
pub struct Annotation {
    pub text: String,
    pub words: Vec<Word>,
}

/// Since there are many words in each text, during serialization we rename each
/// attribute so that when we inpect the generated JSON, it's less cluttered.
#[derive(Serialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Default, Clone))]
pub struct Word {
    #[serde(rename = "w")]
    pub word: String,
    #[serde(rename = "tl")]
    pub top_left: Point,
    #[serde(rename = "br")]
    pub bottom_right: Point,
}

#[derive(Serialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Default, Clone))]
pub struct Point {
    x: i32,
    y: i32,
}

impl Annotation {
    fn from(annotation: GAnnotation) -> Option<Self> {
        let text = annotation.text?;
        let words = annotation
            .pages?
            .into_iter()
            .filter_map(|p| p.blocks)
            .flatten()
            .filter_map(|b| b.paragraphs)
            .flatten()
            .filter_map(|p| p.words)
            .flatten()
            .filter_map(Word::from)
            .collect();

        Some(Self { text, words })
    }
}

impl Word {
    fn from(word: GWord) -> Option<Self> {
        // finds the rectangle around the word
        let vertices = word.bounding_box?.vertices?;
        let top = vertices.iter().min_by(|a, b| a.y.cmp(&b.y))?.y?;
        let bottom = vertices.iter().max_by(|a, b| a.y.cmp(&b.y))?.y?;
        let left = vertices.iter().min_by(|a, b| a.x.cmp(&b.x))?.x?;
        let right = vertices.iter().max_by(|a, b| a.x.cmp(&b.x))?.x?;

        // and collects all the symbols of the word
        let text: String =
            word.symbols?.into_iter().filter_map(|s| s.text).collect();

        (!text.is_empty()).then(|| Self {
            word: text,
            top_left: Point { y: top, x: left },
            bottom_right: Point {
                y: bottom,
                x: right,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use serde_json::json;

    use super::*;

    #[test]
    fn it_filters_out_words_without_vertices_or_symbols() {
        let word = serde_json::from_value(json!({
            "boundingBox": {
                "vertices": [{"x": 1}]
            },
            "symbols": [{"text": "h"}]
        }))
        .unwrap();
        assert_eq!(None, Word::from(word));

        let word = serde_json::from_value(json!({
            "boundingBox": {
                "vertices": [{"x": 1, "y": 2}, {"x": 5, "y": 4}]
            },
            "symbols": []
        }))
        .unwrap();
        assert_eq!(None, Word::from(word));
    }

    #[test]
    fn it_constructs_word() {
        let word = serde_json::from_value(json!({
            "boundingBox": {
                "vertices": [
                    {"x": 1, "y": 5},
                    {"x": 3, "y": 0},
                    {"x": 5, "y": 4}
                ]
            },
            "symbols": [{"text": "h"}, {"text": "w"}]
        }))
        .unwrap();

        assert_eq!(
            Some(Word {
                word: "hw".to_string(),
                top_left: Point { x: 1, y: 0 },
                bottom_right: Point { x: 5, y: 5 },
            }),
            Word::from(word)
        );
    }

    #[test]
    fn it_ignores_no_pages_or_text() {
        let annotation: GAnnotation = serde_json::from_value(json!({
            "text": "hw"
        }))
        .unwrap();
        assert_eq!(None, Annotation::from(annotation));

        let annotation: GAnnotation = serde_json::from_value(json!({
            "pages": []
        }))
        .unwrap();
        assert_eq!(None, Annotation::from(annotation));
    }

    #[test]
    fn it_constructs_text() {
        let gen_word = |t| {
            json!({
                "boundingBox": {
                    "vertices": [{"x": 0, "y": 0}]
                },
                "symbols": [{"text": t}]
            })
        };
        let annotation: GAnnotation = serde_json::from_value(json!({
            "text": "1 2 3 4 5 6 7 8",
            "pages": [
                {"blocks": [
                    {"paragraphs": [{"words": [gen_word("1"), gen_word("2")]}]},
                    {"paragraphs": [{"words": [gen_word("3"), gen_word("4")]}]}
                ]},
                {"blocks": [
                    {"paragraphs": [{"words": [gen_word("5"), gen_word("6")]}]},
                    {"paragraphs": [{"words": [gen_word("7"), gen_word("8")]}]}
                ]}
            ]
        }))
        .unwrap();

        let gen_word = |t: &str| Word {
            word: t.to_string(),
            top_left: Point { x: 0, y: 0 },
            bottom_right: Point { x: 0, y: 0 },
        };
        assert_eq!(
            Some(Annotation {
                text: "1 2 3 4 5 6 7 8".to_string(),
                words: vec![
                    gen_word("1"),
                    gen_word("2"),
                    gen_word("3"),
                    gen_word("4"),
                    gen_word("5"),
                    gen_word("6"),
                    gen_word("7"),
                    gen_word("8"),
                ]
            }),
            Annotation::from(annotation)
        );
    }

    /// I define env with:
    ///
    /// ```bash
    /// export TEST_GCP_SECRET="$(cat .env.google.json)"
    /// ```
    #[ignore]
    #[tokio::test]
    async fn it_uses_gpc_ocr() {
        let gcp_secret = env::var("TEST_GCP_SECRET").unwrap();
        let vision = new(&gcp_secret).await.unwrap();
        let image_url =
        "https://upload.wikimedia.org/wikipedia/commons/d/d9/Plain_text.png".to_string();

        let annotation = vision.annotate(image_url).await.unwrap();
        let _json = serde_json::to_string(&annotation).unwrap();
    }
}
