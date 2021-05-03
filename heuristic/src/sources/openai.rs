use crate::models::Phrase;
use crate::parse;
use crate::prelude::*;
use serde::Deserialize;
use serde_json::json;
use shared::http;

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
) -> Result<Option<Vec<f64>>, Error> {
    let phrases = phrases
        .iter()
        .map(|p| {
            let search = p.avg_estimate() > 0.7 || p.top_word_estimate() > 0.5;
            (search, p)
        })
        .collect::<Vec<_>>();

    let search_text = phrases
        .iter()
        .filter_map(|(search, p)| {
            if *search {
                Some(p.text.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("");

    if search_text.is_empty() {
        return Ok(None);
    }

    // https://beta.openai.com/docs/api-reference/completions/create
    let req = json!({
      "prompt": format!("{} TO GET DISCOUNT USE VOUCHER CODE:", search_text),
      "max_tokens": 8, // *4 to get ~# of characters
      "temperature": 0, // get the best result, no creativity
      "presence_penalty": 0, // we want the same token returned
      "frequency_penalty": 0, // we want the same token returned
    });

    let text = http_client
        .post_json(&conf.openai_completion_url, &req)
        .await
        .map_err(Error::from)
        .and_then(|bytes| {
            serde_json::from_slice::<OpenAiResponseBody>(&bytes)
                .map_err(Error::from)
        })?
        .choices
        .into_iter()
        .next()
        .ok_or_else(|| Error::new("Empty response from OpenAI"))?
        .text;

    // usually GPT-3 returns the voucher as the first word and then just a pile
    // or rubbish
    if let Some(ref first_word) =
        parse::words_from_phrase(&text).into_iter().next()
    {
        Ok(Some(
            phrases
                .into_iter()
                .map(|(search, p)| {
                    p.words.iter().map(move |w| {
                        if search && &w.text == first_word {
                            1.0
                        } else {
                            // unlikely to be a voucher if the AI found another
                            0.2
                        }
                    })
                })
                .flatten()
                .collect(),
        ))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{Phrases, Source};

    use super::*;
    use shared::reqwest::{self, header};
    use std::env;

    #[ignore]
    #[tokio::test]
    async fn it_fetches_estimates_from_openai() {
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

        assert_eq!(word_estimates(&conf, &http_client, &[]).await, Ok(None));

        let mut phrases = Phrases::from_text(
            "Now that you’re signed up, you'll get\n\
                       exclusive discounts, new experiences, competitions\n\
                       and so much more!\n£10 OFF\n\
                       your next order as a special welcome gift\n\
                       CRMGCC9YX5TY\nSimply use this code\n\
                       at the checkout!\n\
                       SHOP NOW",
        );

        // otherwise it would be skipped
        phrases
            .apply_phrases_estimates(
                Source::Dealc,
                vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            )
            .unwrap();

        let estimates = word_estimates(&conf, &http_client, phrases.inner())
            .await
            .unwrap()
            .unwrap();

        let words = phrases.words_str();
        assert_eq!(words.len(), estimates.len());
        assert_eq!(
            &estimates,
            &[
                0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2,
                0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 1.0, 0.2,
                0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2
            ]
        );

        let (index, _) = estimates
            .into_iter()
            .enumerate()
            .find(|(_, e)| *e == 1.0)
            .unwrap();
        assert_eq!(words[index], "CRMGCC9YX5TY");
    }
}
