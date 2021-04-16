//! https://cloud.google.com/vision/quotas
//! https://cloud.google.com/vision/docs/supported-files

use crate::prelude::*;
use futures::executor;
use google_vision1::api::{
    AnnotateImageRequest, BatchAnnotateImagesRequest, Feature, Image,
    ImageSource, TextAnnotation as GAnnotation, Word as GWord,
};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use oauth2::InstalledFlowAuthenticator;
use serde::Serialize;

pub type Vision =
    google_vision1::Vision<hyper::Client<HttpsConnector<HttpConnector>>>;

pub trait Ocr {
    /// Given a URL for an image, it sends it to GCP Vision APIs and performs
    /// OCR annotation.
    fn annotate(&self, image_url: String) -> Result<Option<Annotation>, Error>;
}

/// Creates a new Google client, ready to be used for querying the Vision APIs.
pub async fn new(conf: &Conf) -> Result<Vision, Error> {
    let secret = oauth2::parse_application_secret(&conf.gcp_secret)?;
    let auth = InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .build()
    .await?;

    let hub = Vision::new(
        hyper::Client::builder().build(HttpsConnector::with_native_roots()),
        auth,
    );

    Ok(hub)
}

impl Ocr for Vision {
    fn annotate(&self, image_url: String) -> Result<Option<Annotation>, Error> {
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

        // I'd love to use async_trait, but the [`Authenticator`] type used with
        // the [`Vision`] is not `Send` nor `Sync`
        let (_, data) =
            executor::block_on(self.images().annotate(annotate_req).doit())?;
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

#[derive(Serialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Default, Clone))]
pub struct Word {
    pub word: String,
    pub top_left: (i32, i32),
    pub bottom_right: (i32, i32),
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
            top_left: (left, top),
            bottom_right: (right, bottom),
        })
    }
}

#[cfg(test)]
mod tests {
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
                top_left: (1, 0),
                bottom_right: (5, 5),
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
            top_left: (0, 0),
            bottom_right: (0, 0),
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
}
