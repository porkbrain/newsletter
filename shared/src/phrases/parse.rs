use crate::vision::{self, Annotation, Word};
use std::fmt::{self, Display};

const PUNCTUATION: &[char] = &['?', '!', '.'];

#[derive(Debug)]
enum Sentence<'a> {
    Full {
        words: Vec<&'a Word>,
    },
    /// Marks a large gap between sentences which will prevent them from being
    /// merged into one output offer when running sieve algorithm.
    BreakSentences,
}

#[derive(Debug)]
struct Block<'a> {
    sentences: Vec<&'a Sentence<'a>>,
    bbox: BoundingBox,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BoundingBox {
    top: i32,
    left: i32,
    bottom: i32,
    right: i32,
}

pub fn lines_from_ocr(annotation: &Annotation) -> Vec<String> {
    let all_words: Vec<_> = annotation.words.iter().collect();
    let sentences = sentences_from_words(&annotation.text, &all_words);

    // splits the sentences by <br>, basically version of
    // `sentences.split_mut(Sentence::BreakSentences)`
    let mut blocks = {
        let mut blocks: Vec<Block> = vec![];
        let mut block_index = 0;
        for sentence in &sentences {
            match sentence {
                Sentence::Full { .. } => {
                    if let Some(b) = blocks.get_mut(block_index) {
                        b.push(sentence);
                    } else {
                        blocks.push(sentence.into());
                    }
                }
                Sentence::BreakSentences => {
                    block_index += 1;
                }
            }
        }

        blocks
    };

    // Looks for blocks in the document which align with each other. Alignement
    // is defined as being very close vertically and one of the blocks must
    // contain the other one (within some small threshold).
    let mut merged_blocks = vec![];
    while !blocks.is_empty() {
        let mut cb = blocks.remove(0);
        blocks.retain(|b| {
            if cb.is_aligned_with(b) {
                cb.append(b);
                false
            } else {
                true
            }
        });
        merged_blocks.push(cb);
    }

    // and finally rerun the algorithm for picking sentences, because now we
    // know which sentences are aligned across larger distances
    let mut output = vec![];
    for block in merged_blocks {
        let sentences_text = block
            .sentences
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        let words: Vec<_> = block
            .sentences
            .iter()
            .map(|s| s.words().iter().map(|w| *w))
            .flatten()
            .collect();

        for s in sentences_from_words(&sentences_text, &words) {
            match s {
                // since we've already checked that they are aligned as blocks,
                // we can put them together as sentences
                Sentence::BreakSentences => (),
                Sentence::Full { .. } => output.push(s.to_string()),
            }
        }

        output.push(Sentence::BreakSentences.to_string());
    }

    output
}

fn sentences_from_words<'a>(
    text: &str,
    words: &'a [&Word],
) -> Vec<Sentence<'a>> {
    if words.is_empty() {
        return Vec::new();
    }

    let mut sentences = vec![];
    let mut csentence: Option<Sentence> = None;
    let mut text_chars = text.chars();

    // last word of the slice is handled after the for loop
    for words in words.windows(2) {
        debug_assert_eq!(words.len(), 2);
        let c = words[0];
        let n = words[1];

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

    // Since we're using windows(2), the last word is never pushed into any
    // sentence during the loop. However, the loop left us a signal whether we
    // should push the last word into the last sentence, or the last word should
    // be excluded in a new sentence.
    let last_word = words[words.len() - 1];
    if let Some(mut last_sentence) = csentence.take() {
        last_sentence.push(last_word);
        sentences.push(last_sentence);
    } else {
        sentences.push(last_word.into());
    }

    sentences
}

impl<'a> Block<'a> {
    fn push(&mut self, s: &'a Sentence<'a>) {
        self.bbox = self.bbox.merge(&s.bounding_box());
        self.sentences.push(s);
    }

    fn is_aligned_with(&self, other: &Self) -> bool {
        let last_sentence = self.sentences.iter().last().unwrap();

        let horizontally_aligned = || {
            let horizontal_threshold = last_sentence.avg_word_spacing() * 4;

            let a_contains_b = |a: BoundingBox, b: BoundingBox| {
                (a.left - horizontal_threshold) < b.left
                    && (a.right + horizontal_threshold) > b.right
            };

            let self_contains_other = || a_contains_b(self.bbox, other.bbox);
            let other_contains_self = || a_contains_b(other.bbox, self.bbox);

            self_contains_other() || other_contains_self()
        };

        let vertically_aligned = || {
            let vertical_threshold = last_sentence.line_height();

            (self.bbox.bottom - other.bbox.top).abs() * 2
                < vertical_threshold * 3
        };

        horizontally_aligned() && vertically_aligned()
    }

    fn append(&mut self, other: &Self) {
        self.sentences.extend(other.sentences.iter());
        self.bbox = other
            .sentences
            .iter()
            .fold(self.bbox, |bbox, s| bbox.merge(&s.bounding_box()));
    }
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

    fn bounding_box(&self) -> BoundingBox {
        self.words()
            .iter()
            .fold(self.words()[0].top_left.into(), |cbox, word| {
                cbox.add(word.top_left).add(word.bottom_right)
            })
    }
}

impl BoundingBox {
    fn merge(mut self, other: &Self) -> Self {
        self.top = self.top.min(other.top);
        self.left = self.left.min(other.left);

        self.right = self.right.max(other.right);
        self.bottom = self.bottom.max(other.bottom);

        self
    }

    fn add(self, point: vision::Point) -> Self {
        self.merge(&point.into())
    }
}

impl<'a> From<&'a Sentence<'a>> for Block<'a> {
    fn from(s: &'a Sentence<'a>) -> Self {
        Self {
            bbox: s.bounding_box(),
            sentences: vec![s],
        }
    }
}

impl<'a> From<&'a Word> for Sentence<'a> {
    fn from(w: &'a Word) -> Self {
        Self::Full { words: vec![w] }
    }
}

impl From<vision::Point> for BoundingBox {
    fn from(p: vision::Point) -> Self {
        Self {
            top: p.y,
            bottom: p.y,
            left: p.x,
            right: p.x,
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    #[test]
    fn it_calculates_sentence_bounding_box_and_merges_bounding_boxes() {
        let expected_bounding_box = BoundingBox {
            top: 0,
            left: 0,
            right: 280,
            bottom: 105,
        };

        let word1 = Word {
            word: String::new(),
            top_left: (0, 0).into(),
            bottom_right: (100, 100).into(),
        };

        let word2 = Word {
            word: String::new(),
            top_left: (110, 15).into(),
            bottom_right: (210, 100).into(),
        };

        let word3 = Word {
            word: String::new(),
            top_left: (250, 13).into(),
            bottom_right: (280, 105).into(),
        };

        let sentence = Sentence::Full {
            words: vec![&word1, &word2, &word3],
        };

        assert_eq!(sentence.bounding_box(), expected_bounding_box);
        assert_eq!(
            sentence.bounding_box().merge(&expected_bounding_box),
            expected_bounding_box
        );

        assert_eq!(
            sentence.bounding_box().merge(&BoundingBox {
                top: 1,
                left: 20,
                right: 300,
                bottom: 115,
            }),
            BoundingBox {
                top: 0,
                left: 0,
                right: 300,
                bottom: 115,
            }
        );
    }

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

        panic!("{:#?}", lines_from_ocr(&document));
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
