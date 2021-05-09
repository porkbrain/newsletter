use crate::prelude::*;
use futures::future::join_all;
use serde::Deserialize;
use serde_json::json;
use shared::{
    document::{self, Phrase},
    http,
};

#[derive(Debug, Deserialize)]
struct OpenAiResponseBody {
    choices: Vec<OpenAiResponseChoice>,
}
#[derive(Debug, Deserialize)]
struct OpenAiResponseChoice {
    text: String,
}

pub async fn word_estimates(
    conf: &Conf,
    http_client: &dyn http::Client,
    phrases: &[Phrase],
) -> Vec<Option<f64>> {
    let jobs = phrases.iter().map(|p| process_phrase(conf, http_client, p));
    join_all(jobs).await.into_iter().flatten().collect()
}

async fn process_phrase(
    conf: &Conf,
    http_client: &dyn http::Client,
    p: &Phrase,
) -> Vec<Option<f64>> {
    let should_search = p.avg_estimate() > 0.7;
    if should_search {
        search_phrase(conf, http_client, p).await
    } else {
        p.words.iter().map(|_| None).collect()
    }
}

async fn search_phrase(
    conf: &Conf,
    http_client: &dyn http::Client,
    p: &Phrase,
) -> Vec<Option<f64>> {
    // https://beta.openai.com/docs/api-reference/completions/create
    let req = json!({
      "prompt": format!("{}\n\nUse code:", p.text),
      "max_tokens": 8, // *4 to get ~# of characters
      "temperature": 0, // get the best result, no creativity
      "presence_penalty": 0, // we want the same token returned
      "frequency_penalty": 0, // we want the same token returned
    });

    let first_word = http_client
        .post_json(&conf.openai_completion_url, &req)
        .await
        .map_err(Error::from)
        .and_then(|bytes| {
            serde_json::from_slice::<OpenAiResponseBody>(&bytes)
                .map_err(Error::from)
        })
        .and_then(|json| {
            json.choices
                .into_iter()
                .next()
                .ok_or_else(|| Error::new("Empty response from OpenAI"))
        })
        .map(|choice| document::words::from_phrase(&choice.text))
        .map_err(|e| {
            log::error!("Cannot perform OpenAI request due to {}", e);
            e
        })
        .ok()
        .and_then(|words| words.into_iter().next());

    // usually GPT-3 returns the voucher as the first word or rubbish if no
    // voucher
    if let Some((ref first_word, _)) = first_word {
        let mut any_match = false;
        let estimates = p
            .words
            .iter()
            .map(|w| {
                if &w.text == first_word {
                    any_match = true;
                    Some(1.0)
                } else {
                    Some(0.3)
                }
            })
            .collect();

        if any_match {
            return estimates;
        }
    }

    p.words.iter().map(|_| None).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::{
        document::Source,
        reqwest::{self, header},
        Document,
    };
    use std::env;
    use tokio::fs;

    #[ignore]
    #[tokio::test]
    async fn it_fetches_estimates_from_openai() {
        let (conf, http_client) = make_openai_state();

        // 1. empty input results in empty output
        assert_eq!(word_estimates(&conf, &http_client, &[]).await, vec![]);

        // 2. read testing file into document
        let contents =
            fs::read_to_string("test/assets/openai.json").await.unwrap();
        let json = serde_json::from_str(&contents).unwrap();
        let mut document = Document::from_ocr(&json);

        // add estimates to all phrases in the document, otherwise they would
        // be skipped
        super::super::apply_phrases_estimates(
            &mut document,
            Source::Dealc,
            vec![Some(1.0); 8],
        )
        .unwrap();

        let _estimates =
            word_estimates(&conf, &http_client, document.phrases()).await;
    }

    fn make_openai_state() -> (Conf, reqwest::Client) {
        let engine = "curie";
        let conf = Conf {
            openai_completion_url: format!(
                "https://api.openai.com/v1/engines/{}/completions",
                engine
            ),
            ..Default::default()
        };
        let http_client = {
            let mut headers = header::HeaderMap::new();
            let mut auth_value = header::HeaderValue::from_str(&format!(
                "Bearer {}",
                env::var("TEST_OPENAI_KEY").expect("TEST_OPENAI_KEY missing")
            ))
            .unwrap();
            auth_value.set_sensitive(true);
            headers.insert(header::AUTHORIZATION, auth_value);

            reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap()
        };

        (conf, http_client)
    }
}
