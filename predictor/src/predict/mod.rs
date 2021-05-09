mod common_phrases;
mod openai;

use crate::prelude::*;
use shared::http;
use shared::{
    document::{Document, Source},
    vision::Annotation,
};

pub async fn deals_and_vouchers(
    conf: &Conf,
    http_client: &dyn http::Client,
    annotation: &Annotation,
) -> Result<Document, Error> {
    let mut document = Document::from_ocr(annotation);

    apply_dealc_and_voucherc_estimates(conf, http_client, &mut document)
        .await?;

    // if there are some some common newsletter phrases (USE CODE ABC20), then
    // apply estimates from those
    let common_phrases_estimates =
        common_phrases::word_estimates(document.words().as_slice());
    if let Some(estimates) = common_phrases_estimates {
        let estimates = estimates.into_iter().map(Some).collect();
        apply_words_estimates(&mut document, Source::CommonPhrases, estimates)?;
    }

    // send some promising phrases to openai to check them out
    let openai_estimates =
        openai::word_estimates(conf, http_client, document.phrases()).await;
    apply_words_estimates(&mut document, Source::OpenAi, openai_estimates)?;

    Ok(document)
}

async fn apply_dealc_and_voucherc_estimates(
    conf: &Conf,
    http_client: &dyn http::Client,
    document: &mut Document,
) -> Result<(), Error> {
    // fetch estimates for how likely each phrase is a deal
    // TODO: can be made concurrent with next step
    let dealc_estimates: Vec<_> = {
        let phrases_json = serde_json::to_value(&document.phrases_str())?;
        let dealc_res_body = http_client
            .post_json(&conf.dealc_url, &phrases_json)
            .await?;

        let estimates: Vec<f64> = serde_json::from_slice(&dealc_res_body)?;
        estimates.into_iter().map(Some).collect()
    };
    apply_phrases_estimates(document, Source::Dealc, dealc_estimates)?;

    // fetch estimates for how likely each word is a voucher
    let voucherc_estimates: Vec<_> = {
        let words_json = serde_json::to_value(document.words_str())?;
        let voucherc_res_body = http_client
            .post_json(&conf.voucherc_url, &words_json)
            .await?;

        let estimates: Vec<f64> = serde_json::from_slice(&voucherc_res_body)?;
        estimates.into_iter().map(Some).collect()
    };
    apply_words_estimates(document, Source::Voucherc, voucherc_estimates)?;

    Ok(())
}

pub fn apply_phrases_estimates(
    document: &mut Document,
    source: Source,
    estimates: Vec<Option<f64>>,
) -> Result<(), Error> {
    let phrases = document.phrases_mut();
    if phrases.len() != estimates.len() {
        return Err(Error::new(format!(
            "Got {} phrases, but {} estimates",
            phrases.len(),
            estimates.len()
        )));
    }

    for (phrase, estimate) in phrases.into_iter().zip(estimates.into_iter()) {
        if let Some(estimate) = estimate {
            phrase.estimates.insert(source, estimate);
        }
    }

    Ok(())
}

pub fn apply_words_estimates(
    document: &mut Document,
    source: Source,
    estimates: Vec<Option<f64>>,
) -> Result<(), Error> {
    let words = document.words_mut();
    if words.len() != estimates.len() {
        return Err(Error::new(format!(
            "Got {} words, but {} estimates",
            words.len(),
            estimates.len()
        )));
    }

    for (w, estimate) in words.into_iter().zip(estimates.into_iter()) {
    if let Some(estimate) = estimate {
            w.estimates.insert(source, estimate);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use shared::reqwest;

    #[ignore]
    #[tokio::test]
    async fn it_applies_dealc_and_voucherc_estimates() {
        let ocr_path = "";

        let conf = Conf {
            dealc_url: "http://localhost:8081".to_string(),
            voucherc_url: "http://localhost:8080".to_string(),
            ..Default::default()
        };
        let http_client = reqwest::Client::new();
        let contents = fs::read_to_string(ocr_path).unwrap();
        let annotation = serde_json::from_str(&contents).unwrap();
        let mut phrases = Document::from_ocr(&annotation);

        apply_dealc_and_voucherc_estimates(&conf, &http_client, &mut phrases)
            .await
            .expect("Cannot get phrases with estimates");

        panic!("{:#?}", phrases);
    }

    #[test]
    fn it_serializes() {
        let json = json!({
            "text": "first phrase and then\nthere second phrase",
            "words": [
                {
                    "tl": {"x": 0, "y": 0},
                    "br": {"x": 0, "y": 0},
                    "w": "first",
                },
                {
                    "tl": {"x": 0, "y": 0},
                    "br": {"x": 0, "y": 0},
                    "w": "phrase",
                },
                {
                    "tl": {"x": 0, "y": 0},
                    "br": {"x": 0, "y": 0},
                    "w": "and",
                },
                {
                    "tl": {"x": 0, "y": 0},
                    "br": {"x": 0, "y": 0},
                    "w": "then",
                },
                {
                    "tl": {"x": 100, "y": 0},
                    "br": {"x": 100, "y": 0},
                    "w": "there",
                },
                {
                    "tl": {"x": 100, "y": 0},
                    "br": {"x": 100, "y": 0},
                    "w": "second",
                },
                {
                    "tl": {"x": 100, "y": 0},
                    "br": {"x": 100, "y": 0},
                    "w": "phrase",
                },
            ]
        });
        let mut document =
            Document::from_ocr(&serde_json::from_value(json).unwrap());

        apply_phrases_estimates(
            &mut document,
            Source::Dealc,
            vec![Some(0.8), Some(0.0), Some(0.7), Some(0.0)],
        )
        .unwrap();
        apply_words_estimates(
            &mut document,
            Source::OpenAi,
            vec![Some(0.5); 9]
        )
        .unwrap();

        assert_eq!(
            json!([
                    {
                        "text": "first phrase and then",
                        "estimates": { "dealc": 0.8 },
                        "words": [
                            {
                                "text": "first",
                                "estimates": { "open_ai": 0.5 },
                            },
                            {
                                "text": "phrase",
                                "estimates": { "open_ai": 0.5 },
                            },
                            {
                                "text": "and",
                                "estimates": { "open_ai": 0.5 },
                            },
                            {
                                "text": "then",
                                "estimates": { "open_ai": 0.5 },
                            },
                        ]
                    },
                    {
                        "text": "<br>",
                        "estimates": { "dealc": 0.0 },
                        "words": [
                            {
                                "text": "<br>",
                                "estimates": { "open_ai": 0.5 },
                            },
                        ]
                    },
                    {
                        "text": "there second phrase",
                        "estimates": { "dealc": 0.7 },
                        "words": [
                            {
                                "text": "there",
                                "estimates": { "open_ai": 0.5 },
                            },
                            {
                                "text": "second",
                                "estimates": { "open_ai": 0.5 },
                            },
                            {
                                "text": "phrase",
                                "estimates": { "open_ai": 0.5 },
                            },
                        ]
                    },
                    {
                        "text": "<br>",
                        "estimates": { "dealc": 0.0 },
                        "words": [
                            {
                                "text": "<br>",
                                "estimates": { "open_ai": 0.5 },
                            },
                        ]
                    },
            ]),
            serde_json::to_value(&document).unwrap()
        );
    }
}
