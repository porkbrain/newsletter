use std::fmt::{self, Display};

use crate::vision::{Annotation, Word};

const PUNCTUATION: &[char] = &['?', '!', '.'];

enum Sentence<'a> {
    Full {
        words: Vec<&'a Word>,
    },
    /// Marks a large gap between sentences which will prevent them from being
    /// merged into one output offer when running sieve algorithm.
    BreakSentences,
}

pub fn lines_from_email(annotation: &Annotation) -> Vec<String> {
    let mut sentences = vec![];
    let mut csentence: Option<Sentence> = None;
    let mut text_chars = annotation.text.chars();

    // TODO: last word is missing, padding to a word that's super far away?
    for words in annotation.words.windows(2) {
        debug_assert_eq!(words.len(), 2);
        let c = &words[0];
        println!("{}", c.word);
        let n = &words[1];

        if let Some(cs) = &mut csentence {
            cs.push(&c);
        } else {
            csentence = Some(c.into());
        }
        let cs = csentence.as_mut().unwrap();

        let are_separated_by_nl = text_chars
            .find(|c| *c == ' ' || *c == '\n')
            .filter(|c| *c == '\n')
            .is_some();

        let cpline_height = cs.line_height();
        let avg_word_spacing = cs.avg_word_spacing();
        let dist_y = n.top_left.y - c.bottom_right.y;

        // If distance y is negative, it means that the word is below the
        // next one. Reasons this can happen:
        // 1. in the same line, it's always going to be negative by ~line
        //      height;
        // 2. the ocr might have weird bouding box, one word is a dot or
        //      something small;
        // 2. the OCR puts words in order based on paragraphs, so a new
        //      paragraph that's on the left of the previous one has
        //      started, and it backtracks a bit up.
        let is_vertically_distant =
            || dist_y > cpline_height * 2 || dist_y < cpline_height * -3;

        let is_horizontally_distant = || {
            let dist_x = n.top_left.x - c.bottom_right.x;

            // if there's a large space between the two consecutive words, they
            // belong to a new box
            let distant_to_the_left =
                || dist_x > avg_word_spacing * 3 && are_separated_by_nl;

            // if the first word of the sentence is much further to the right
            // than the next word, don't connect the sentences
            //
            // ```
            //                        1st w         curr w
            // next w                    |            |
            //  |                        |            |
            //  |                     Some sentence that
            // First column           continues in columns
            // ```
            let distant_to_the_right = || {
                // We must find the beginning of the block, that is the first
                // sentence after the last "<br>". That is because we cannot
                // know whether the last sentence has ended with e.g. an
                // exclamation mark in half of the text. For example, in the
                // following text the current sentence starts with "And", but
                // the x coord of this word is way beyond the x coord of the
                // next word "again.".
                //
                // ```
                //                       curr w
                //                        |
                // This is a text which   |
                // goes on several lines  |
                // but ends. And now starts
                // again.
                //  |
                //  |
                // next w
                // ```
                let block_beginning = sentences
                    .iter()
                    .rev()
                    .take_while(|s| !matches!(s, Sentence::BreakSentences))
                    .last()
                    .map(|s| s.words()[0].top_left)
                    .unwrap_or_else(|| cs.words()[0].top_left);
                block_beginning.x - n.top_left.x > avg_word_spacing * 4
            };

            distant_to_the_left() || distant_to_the_right()
        };

        let has_punctuation = || c.word.ends_with(PUNCTUATION);
        let new_paragraph =
            || !c.word.ends_with(':') && dist_y * 2 > cpline_height * 3;

        if is_vertically_distant() || is_horizontally_distant() {
            csentence.take().map(|p| sentences.push(p));
            sentences.push(Sentence::BreakSentences);
        } else if has_punctuation() || new_paragraph() {
            csentence.take().map(|p| sentences.push(p));
        } else {
            // ... continue the sentence with next iteration
        }
    }

    sentences.into_iter().map(|s| s.to_string()).collect()
}

impl<'a> Sentence<'a> {
    fn push(&mut self, w: &'a Word) {
        let words = self.words_mut();
        words.push(w);
    }

    fn line_height(&self) -> i32 {
        let words = self.words();
        let total = words
            .iter()
            .fold(0, |sum, w| sum + w.bottom_right.y - w.top_left.y);
        total / words.len() as i32
    }

    fn avg_word_spacing(&self) -> i32 {
        let words = self.words();
        if words.len() == 1 {
            let spacing_is_about_n_chars = 2;
            let w = words[0];
            let char_width =
                (w.bottom_right.x - w.top_left.x) / w.word.len() as i32;

            char_width * spacing_is_about_n_chars
        } else {
            let total = words.windows(2).fold(0, |sum, words| {
                let a = words[0];
                let b = words[1];
                sum + (b.top_left.x - a.bottom_right.x).max(0)
            });

            total / (words.len() as i32 - 1)
        }
    }

    fn words_mut(&mut self) -> &mut Vec<&'a Word> {
        match self {
            Self::Full { ref mut words } => words,
            Self::BreakSentences => panic!("Sentence::Br cannot hold words"),
        }
    }

    fn words(&self) -> &[&Word] {
        match self {
            Self::Full { words } => &words,
            Self::BreakSentences => panic!("Sentence::Br cannot hold words"),
        }
    }
}

impl<'a> From<&'a Word> for Sentence<'a> {
    fn from(w: &'a Word) -> Self {
        Self::Full { words: vec![w] }
    }
}

impl<'a> Display for Sentence<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BreakSentences => write!(f, "<br>"),
            Self::Full { words } => {
                let sentence = words
                    .iter()
                    .map(|w| w.word.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                write!(f, "{}", sentence)
            }
        }
    }
}

/// Returns list of sanitized words in first term, and their raw form in second.
pub fn words_from_phrase(phrase: &str) -> Vec<(String, String)> {
    const VOUCHER_MAX_LEN: usize = 32;
    const VOUCHER_MIN_LEN: usize = 3;

    const TRIM_CHARS_FROM_WORD: &[char] =
        &['\t', '"', '\'', ',', '.', '?', '!', ')', '(', ':', '*'];

    phrase
        .replace('\n', " ")
        .split(' ')
        .map(|s| (s.trim().trim_matches(TRIM_CHARS_FROM_WORD), s))
        .filter(|(s, _)| !s.is_empty())
        .filter(|(s, _)| (VOUCHER_MIN_LEN..=VOUCHER_MAX_LEN).contains(&s.len()))
        .map(|(sanitized, raw)| (sanitized.to_string(), raw.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    #[test]
    fn it_calculates_avg_word_spacing() {
        let word1 = Word {
            word: "abcd".to_string(),
            top_left: (0, 0).into(),
            bottom_right: (100, 100).into(),
        };

        let word2 = Word {
            word: String::new(),
            top_left: (110, 0).into(),
            bottom_right: (210, 100).into(),
        };

        let word3 = Word {
            word: String::new(),
            top_left: (250, 0).into(),
            bottom_right: (280, 100).into(),
        };

        let sentence = Sentence::Full {
            words: vec![&word1, &word2, &word3],
        };
        assert_eq!(25, sentence.avg_word_spacing());

        let sentence = Sentence::Full {
            words: vec![&word1],
        };
        assert_eq!(50, sentence.avg_word_spacing());
    }

    #[test]
    fn it_works() {
        let document = testing_document("parse_text1");

        panic!("{:#?}", lines_from_email(&document));
    }

    #[test]
    fn it_works_with_openai_output() {
        assert_correct_words_output(
            words_from_phrase(
                "SUMMER20.\n\
            The",
            ),
            vec!["SUMMER20", "The"],
            vec!["SUMMER20.", "The"],
        );
    }

    #[test]
    fn it_sanitizes_words() {
        assert_correct_words_output(
            words_from_phrase("Hello. "),
            vec!["Hello"],
            vec!["Hello."],
        );
        assert_correct_words_output(
            words_from_phrase("there!"),
            vec!["there"],
            vec!["there!"],
        );
        assert_correct_words_output(
            words_from_phrase("Code:"),
            vec!["Code"],
            vec!["Code:"],
        );
        assert_correct_words_output(
            words_from_phrase("~~nicky.~.~?"),
            vec!["~~nicky.~.~"],
            vec!["~~nicky.~.~?"],
        );
        assert_correct_words_output(
            words_from_phrase("I've"),
            vec!["I've"],
            vec!["I've"],
        );
        assert_correct_words_output(
            words_from_phrase("(been)"),
            vec!["been"],
            vec!["(been)"],
        );
        assert_correct_words_output(
            words_from_phrase("looking"),
            vec!["looking"],
            vec!["looking"],
        );
        assert_correct_words_output(
            words_from_phrase("'you"),
            vec!["you"],
            vec!["'you"],
        );
        assert_correct_words_output(words_from_phrase("'n''"), vec![], vec![]);
        assert_correct_words_output(
            words_from_phrase(
                "thisisextremelylongwordwhichwillneverbeavoucherinmillionyears",
            ),
            vec![],
            vec![],
        );
        assert_correct_words_output(
            words_from_phrase("star*"),
            vec!["star"],
            vec!["star*"],
        );
    }

    fn assert_correct_words_output(
        output: Vec<(String, String)>,
        expected_sanitized: Vec<&str>,
        expected_raw: Vec<&str>,
    ) {
        assert_eq!(
            output.iter().map(|(s, _)| s.as_str()).collect::<Vec<_>>(),
            expected_sanitized
        );
        assert_eq!(
            output.iter().map(|(_, r)| r.as_str()).collect::<Vec<_>>(),
            expected_raw
        );
    }

    pub fn testing_document(name: &str) -> Annotation {
        let curr_path = fs::canonicalize(".").unwrap();
        println!("{:?}", curr_path);

        let test_path = if curr_path.ends_with("shared") {
            PathBuf::from("test/assets")
        } else if curr_path.ends_with("newsletter") {
            PathBuf::from("shared/test/assets")
        } else {
            panic!("Test must be called from /newsletter or /newsletter/shared")
        }
        .join(format!("{}.json", name));

        let contents = fs::read_to_string(&test_path).unwrap_or_else(|e| {
            panic!(
                "Cannot read test file {} at {:?} due to {}",
                name, test_path, e
            )
        });

        serde_json::from_str(&contents).unwrap()
    }
}
