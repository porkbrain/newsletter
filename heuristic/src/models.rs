use crate::parse;
use crate::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Phrases(Vec<Phrase>);

#[derive(Debug)]
pub struct Phrase {
    pub text: String,
    pub estimates: HashMap<Source, f64>,
    pub words: Vec<Word>,
}

#[derive(Debug)]
pub struct Word {
    pub text: String,
    pub estimates: HashMap<Source, f64>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Source {
    Dealc,
    Voucherc,
    OpenAi,
    CommonPhrases,
}

impl Phrases {
    pub fn new(phrases: Vec<Phrase>) -> Self {
        Self(phrases)
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
            phrase.estimates.insert(Source::Dealc, estimate);
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
}

impl Word {
    pub fn new(text: String) -> Self {
        Self {
            text,
            estimates: HashMap::default(),
        }
    }
}
