pub mod parse;
pub mod words;

use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Serialize, Deserialize, Debug)]
pub struct Phrases(Vec<Phrase>);

#[derive(Serialize, Deserialize, Debug)]
pub struct Phrase {
    pub text: String,
    pub estimates: HashMap<Source, f64>,
    pub words: Vec<Word>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Word {
    pub text: String,
    #[serde(skip)]
    pub raw: String,
    pub estimates: HashMap<Source, f64>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    Dealc,
    Voucherc,
    OpenAi,
    CommonPhrases,
}

impl Phrases {
    pub fn from_text(text: &str) -> Self {
        Self(vec![String::new()].into_iter().map(Phrase::new).collect())
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

    pub fn words(&mut self) -> Vec<&Word> {
        self.0.iter().map(|p| &p.words).flatten().collect()
    }

    pub fn words_mut(&mut self) -> Vec<&mut Word> {
        self.0.iter_mut().map(|p| &mut p.words).flatten().collect()
    }
}

impl Phrase {
    pub fn new(text: impl Display) -> Self {
        let text = text.to_string();
        let words = words::from_phrase(&text)
            .into_iter()
            .map(|(s, r)| Word::new_with_raw(s, r))
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

    pub fn top_word(&self) -> Option<&Word> {
        self.words.iter().max_by(|a, b| {
            a.avg_estimate()
                .partial_cmp(&b.avg_estimate())
                .unwrap_or(Ordering::Equal)
        })
    }

    pub fn top_word_estimate(&self) -> f64 {
        self.words
            .iter()
            .map(Word::avg_estimate)
            .fold(0.0, |top, c| if top > c { top } else { c })
    }
}

impl Word {
    pub fn new_with_raw(text: impl Display, raw: impl Display) -> Self {
        let text = text.to_string();
        let raw = raw.to_string();
        Self {
            raw,
            text,
            estimates: HashMap::default(),
        }
    }

    pub fn avg_estimate(&self) -> f64 {
        let total: f64 = self.estimates.values().copied().sum();
        total / self.estimates.len() as f64
    }
}
