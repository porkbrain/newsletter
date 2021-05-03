use crate::parse;
use crate::prelude::*;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct Phrases(Vec<Phrase>);

#[derive(Serialize, Debug)]
pub struct Phrase {
    pub text: String,
    pub estimates: HashMap<Source, f64>,
    pub words: Vec<Word>,
}

#[derive(Debug, Serialize)]
pub struct Word {
    pub text: String,
    pub estimates: HashMap<Source, f64>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    Dealc,
    Voucherc,
    OpenAi,
    CommonPhrases,
}

impl Phrases {
    pub fn from_text(text: &str) -> Self {
        Self(
            parse::lines_from_email(text)
                .into_iter()
                .map(Phrase::new)
                .collect(),
        )
    }

    pub fn inner(&self) -> &[Phrase] {
        self.0.as_slice()
    }

    pub fn inner_mut(&mut self) -> &mut [Phrase] {
        self.0.as_mut_slice()
    }

    pub fn phrases_str(&self) -> Vec<&str> {
        self.0.iter().map(|p| p.text.as_str()).collect()
    }

    pub fn words_str(&self) -> Vec<&str> {
        self.0
            .iter()
            .map(|p| p.words.iter().map(|w| w.text.as_str()))
            .flatten()
            .collect()
    }

    pub fn apply_phrases_estimates(
        &mut self,
        source: Source,
        estimates: Vec<f64>,
    ) -> Result<(), Error> {
        let phrases = self.inner_mut();
        if phrases.len() != estimates.len() {
            return Err(Error::new(format!(
                "Got {} phrases, but {} estimates",
                phrases.len(),
                estimates.len()
            )));
        }

        for (phrase, estimate) in phrases.into_iter().zip(estimates.into_iter())
        {
            phrase.estimates.insert(source, estimate);
        }

        Ok(())
    }

    pub fn apply_words_estimates(
        &mut self,
        source: Source,
        estimates: Vec<f64>,
    ) -> Result<(), Error> {
        let words = self.words_mut();
        if words.len() != estimates.len() {
            return Err(Error::new(format!(
                "Got {} words, but {} estimates",
                words.len(),
                estimates.len()
            )));
        }

        for (w, estimate) in words.into_iter().zip(estimates.into_iter()) {
            w.estimates.insert(source, estimate);
        }

        Ok(())
    }

    fn words_mut(&mut self) -> Vec<&mut Word> {
        self.0.iter_mut().map(|p| &mut p.words).flatten().collect()
    }
}

impl Phrase {
    pub fn new(text: String) -> Self {
        let words = parse::words_from_phrase(&text)
            .into_iter()
            .map(Word::new)
            .collect();

        Self {
            text,
            words,
            estimates: HashMap::default(),
        }
    }

    pub fn avg_estimate(&self) -> f64 {
        let total: f64 = self.estimates.values().copied().sum();
        total / self.estimates.len() as f64
    }

    pub fn top_word_estimate(&self) -> f64 {
        self.words
            .iter()
            .map(Word::avg_estimate)
            .fold(0.0, |top, c| if top > c { top } else { c })
    }
}

impl Word {
    pub fn new(text: String) -> Self {
        Self {
            text,
            estimates: HashMap::default(),
        }
    }

    pub fn avg_estimate(&self) -> f64 {
        let total: f64 = self.estimates.values().copied().sum();
        total / self.estimates.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_serializes() {
        let mut phrases =
            Phrases::from_text("first phrase and then\nthere is second phrase");
        phrases
            .apply_phrases_estimates(Source::Dealc, vec![0.8, 0.7])
            .unwrap();
        phrases
            .apply_words_estimates(
                Source::OpenAi,
                (0..7).map(|_| 0.5).collect(),
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
                        "text": "there is second phrase",
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
                    }
            ]),
            serde_json::to_value(&phrases).unwrap()
        );
    }
}
