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

pub fn from_ocr(annotation: &Annotation) -> Vec<String> {
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
    fn it_parses_text3() {
        let document = testing_document("parse_text3");
        let expected_output = vec![
            "9 affiliate",
            "UK EDITION What\'s Hot Newsletter - 6th May, 2021",
            "Hello Affiliate,",
            "<br>",
            "Welcome to another installment of the CJ What\'s Hot \
            Newsletter (UK Edition).",
            "<br>",
            "We\'ve rounded up some of this week\'s hottest offers \
            from our Advertisers and want to",
            "introduce you to some of our newly-launched Advertisers.",
            "<br>",
            "Upcoming Occasions • 12th - Eid al-Fitr • 15th - FA Cup Final \
            | English Wine Week begins • 18th - RHS Chelsea Flower Show begins",
            "<br>",
            "Please ensure that you replace the PID within the AID links \
            to your website ID (PID).",
            "<br>",
            "THE KOOPLES PARIS",
            "<br>",
            "The Kooples (CID: 5102672) Contact: Özge Torbacioglu • Offer: \
            The Kooples is here to brighten your days with our \
            Mid-Season Sale I Up to 40% off your Spring/Summer \
            essentials • Vertical: Fashion • Code: No Code Necessary \
            • Dates: Until 13th May, 2021 • T&C\'s: NA • AID: 14518049",
            "<br>",
            "Contact the CJ account manager for opportunities.",
            "<br>",
            "GET LINKS",
            "<br>",
            "Domino\'s (CID: 5019299) Contact: Andrea De Luca",
            "Vertical: Food • Offer 1: May Nationwide 35% off when you \
            spend £40 online • Code: CDNRZGXQ • Dates: Until 31st \
            May, 2021 • T&C\'s: Cannot be used in conjunction with \
            any other offer • AID: 14510192",
            "<br>",
            "* Domino\'s",
            "<br>",
            "• Offer 2: May Nationwide - 25% off when you spend £30 \
            online • Code: INHSZRWI • Dates: Until 31st May, 2021 \
            • T&C\'s: Cannot be used in conjunction with any other \
            offer • AID: 14510194",
            "<br>",
            "Contact the CJ account manager for opportunities.",
            "<br>",
            "GET LINKS",
            "<br>",
            "Ray-Ban (CID 5314775) Contact: CJ Luxottica Team",
            "Vertical: Retail • Offer 1: Enjoy up to 50% off selected \
            styles at Ray-Ban.com + free shipping • Dates: Until \
            31st December, 2021 • T&C\'s: Not cumulable with other \
            offers • AID: 14400078",
            "<br>",
            "Ray-Ban OENUINE INCE 137",
            "<br>",
            "• Offer 2: Enjoy up to 45£ off polar lenses with code \
            RBPOLAR at checkout on Ray-Ban.com +",
            "free shipping • Code: RBPOLAR • Dates: Until 9th May, 2021 \
            • T&C\'s: Not cumulable with other offers • AID: 14510226",
            "<br>",
            "Contact the CJ account manager for opportunities.",
            "<br>",
            "GET LINKS",
            "<br>",
            "Oakley (CID 5314779) Contact: CJ Luxottica Team Vertical: \
            Retail • Offer 1: Save £30 off polarized sunglasses at \
            the Official Oakley online store + Free Shipping • Dates: \
            Until 9th May, 2021 • T&C\'s: Not cumulable with other \
            offers • AID: 14510303",
            "<br>",
            "O AKLE Y",
            "<br>",
            "• Offer 2: Score up to 50% off selected eyewear @ \
            Oakley.com + Free Shipping.",
            "• Dates: Until 30th June, 2021 • T&C\'s: Not cumulable \
            with other offers • AID: 14080480",
            "<br>",
            "Contact the CJ account manager for opportunities.",
            "<br>",
            "GET LINKS",
            "<br>",
            "sunglass hut",
            "<br>",
            "Sunglass Hut (CID 5314784) Contact: CJ Luxottica Team",
            "Vertical: Retail • Offer 1: Get more for less: 50% off \
            second pair with code SECOND50 at checkout • Dates: \
            Until 9th May, 2021 • T&C\'s: Not cumulable with other \
            offers • AID: 14511255",
            "<br>",
            "• Offer 2: Get up to 50% off selected styles + free \
            shipping @ SunglassHut.com • Dates: Until 30th June, \
            2021 • T&C\'s: Not cumulable with other offers • AID: 14084900",
            "<br>",
            "Contact the CJ account manager for opportunities.",
            "<br>",
            "GET LINKS",
            "<br>",
            "Premier TEFL (CID: 5699020)",
            "Contact: Megan Cronin Vertical: Education • Offer 1: \
            Teach in Taiwan • Dates: Applications Open Now • \
            T&C\'s: See website for eligibility criteria • AID: 14511392",
            "<br>",
            "PREMIER TEFL",
            "<br>",
            "• Offer 2: Teach in South Korea • Dates: Applications Open \
            Now • T&C\'s: See website for eligibility criteria • AID: 14507759",
            "<br>",
            "• Offer 3: Teach in Argentina • Dates: Applications Open Now \
            • T&C\'s: See website for eligibility criteria • AID: 14511223",
            "<br>",
            "• Offer 4: Teach in Romania • Dates: Applications Open Now • \
            T&C\'s: See website for eligibility criteria • AID: 14511247",
            "<br>",
            "• Offer 5: Teach in Germany • Dates: Applications Open Now • \
            T&C\'s: See website for eligibility criteria • AID: 14511301",
            "<br>",
            "• Offer 6: Teach in Thailand • Dates: Applications Open Now • \
            T&C\'s: See website for eligibility criteria • AID: 14511105",
            "<br>",
            "Contact the CJ account manager for opportunities.",
            "<br>",
            "GET LINKS",
            "<br>",
            "A ENVIROBUILD",
            "<br>",
            "EnviroBuild UK (CID: 5633485)",
            "Contact: Liz Ponsford",
            "Vertical: Home and Garden • Offer : 10% Off Luxury Rattan \
            Garden Furniture until 10th May.",
            "Every purchase supports The Rainforest Trust Free Delivery \
            and Free Returns.",
            "On site • Code: On Site • Dates: Until 10th May, 2021 • T&C\'s: \
            UK Only • AID: 14518625",
            "<br>",
            "Contact the CJ account manager for opportunities.",
            "<br>",
            "GET LINKS",
            "<br>",
            "Storypod",
            "<br>",
            "Storypod INT (CID: 5723599) Contact: Taji Nizar",
            "• Offer: 20% OFF Everything!",
            "• Vertical: Kid\'s Toys • Code: FIRST20 • Dates: Until 30th May, \
            2021 • T&C\'s: NA • AID: 14518866",
            "<br>",
            "New Launch Introduction: Storypod is the hottest new \
            storytelling audio toy targeting millennial parents \
            with kids aged 3+.",
            "Storypod\'s promise is simple: Less screen time, more story time.",
            "The interactive Storypod is activated via NFC- technology by \
            lovable yarn characters - called Crafties - as well as \
            read-along audio books and trivia cards that feature hours \
            and hours of audio edutainment and fun.",
            "<br>",
            "Commission Rate: 25% New Customers 20% Returning Customers \
            Referral Period: 14 Days (Shopify)",
            "<br>",
            "Contact the CJ account manager for opportunities.",
            "<br>",
            "GET LINKS",
            "<br>",
            "LOG IN TO YOUR CJ ACCOUNT TO BROWSE OFFERS",
            "<br>",
            "Have a Question?",
            "<br>",
            "Please contact us at whatshot uk@cj.com should you have \
            any questions or feedback.",
            "<br>",
            "Sincerely, Your CJ Team",
            "<br>",
            "O y in f",
            "© 2021 CJ Affiliate CJ Affiliate | 2 Television Centre 3rd \
            101 Wood Lane, London W12 7FR | UK This email was sent \
            to affiliate.uk@joingsg.com View Online | Terms of Use \
            | Privacy Policy Update Email Preferences Unsubscribe",
            "<br>",
        ];

        assert_eq!(expected_output, from_ocr(&document));
    }

    #[test]
    fn it_parses_text2() {
        let document = testing_document("parse_text2");
        let expected_output =
            vec![
            "TaskRabbit",
            "<br>",
            "YOUR Affiliate",
            "<br>",
            "Newsletter",
            "<br>",
            "Hope everyone is doing well.",
            "We\'ve extended our promo codes through May offering $10 \
                off for new TaskRabbit Clients.",
            "A few tasks to highlight this month include: Moving, \
            Mounting, Furniture Assembly, Lawn and Garden Care as well \
            as Spring Cleaning.", "Please find promo codes and \
            expiration dates listed below:",
            "<br>", "NEW10US $10 OFF*", "<br>", "NEW10UK £10 OFF*", "<br>",
            "NEW10CA $10 OFF*", "<br>",
            "*Expires: 5/31/2021, new clients only.",
            "One time use.", "<br>",
            "Reach out to our team with any inquiries about pulling links \
                or how to receive commissions by promoting TaskRabbit!",
                "<br>", "Connect With Us",
            "<br>",
            "Keep an eye on our affiliate newsletters for the latest updates \
                 and promotions, so you can be the first to post our links!",
                 "<br>", "GOOD NEWS IS COMING", "<br>",
            "Category Highlights", "<br>", "Moving", "<br>", "Mounting",
            "<br>", "Furniture Assembly", "<br>", "Lawn & Garden Care",
            "<br>",
            "Please reach out to our team with any questions, \
                we\'re here to help!", "<br>", "Connect With Us", "<br>",
                "IKEA Patnered with IKEA",
            "<br>",
            "message was sent by an advertiser in the CJ Affiliate network \
                based on the mail settings selected in your account.",
            "<br>",
            "from all messages from this advertiser.", "<br>",
            "Log in and view your Mail Settings and edit your \
                messaging_preferences.",
            "For assistance with your account, use the Support Center \
            in the upper right side of the CJ Account Manager.",
            "CJ Affiliate 530 East Montecito Street Santa Barbara, CA \
            93103 United States",
            "<br>"];

        assert_eq!(expected_output, from_ocr(&document));
    }

    #[test]
    fn it_parses_text1() {
        let document = testing_document("parse_text1");
        let expected_output = vec![
            "Is this email not displaying correctly?",
            "Click here to view on our website.",
            "<br>",
            "●Optimus pm gen",
            "<br>",
            "Snapfish UK",
            "Affiliate Newsletter",
            "<br>",
            "Ò Snapfish",
            "<br>",
            "SNAP.",
            "PRINT.",
            "SAVE.",
            "<br>",
            "New Voucher Codes at Snapfish!",
            "<br>",
            "Voucher Codes",
            "<br>",
            "#50% OFF UP PRINTS + POSTERS 50% 30% 150- prints",
            "1-149 prints",
            "<br>",
            "Up to 50% off Prints & Posters: 50% off 150+ prints / \
                30% off 1-149 prints",
            "Code - PRINT521 09-05-2021",
            "<br>",
            "Use code PRINTS21 by 9/5.",
            "<br>",
            "ORDER PRINTS",
            "<br>",
            "50% off photo books (excludes extra pages) Code - \
                BOOK521 31-05-2021",
            "<br>",
            "Use code PRINT521 for up to 50% off prints and posters.",
            "Code expires 09-05-2021",
            "<br>",
            "Up to 50% off Prints & Posters: 50% off 150+ prints / \
                30% off 1-149 prints",
            "Code - SFUK1088 31-05-2021",
            "<br>",
            "50% OFF PHOTO BOOKS \"Excludes extra pages",
            "<br>",
            "AVAVAYA",
            "<br>",
            "Use code BOOKS21 by 9/5.",
            "<br>",
            "CREATE YOURS",
            "<br>",
            "50% off photo books (excludes extra pages) Code - \
                SFUK1089 31-05-2021",
            "<br>",
            "Use code BOOK521 for 50% off photo books (excluding extra pages).",
            "Code expires 09-05-2021",
            "<br>",
            "We also have the following affiliate codes, expiring 31-05-2021:",
            "<br>",
            "50% off Canvas Prints (including sets and split canvas) \
                Code - SFUK1090 31-05-2021",
            "<br>",
            "SFUK1088 - Up to 50% off Prints & Posters: 50% off 150+ prints / \
                30% off 1-149 prints",
            "<br>",
            "SFUK1089 - 50% off Photo Books (Excludes extra pages)",
            "<br>",
            "SFUK1090 - 50% off Canvas Prints (including sets and \
            split canvas)",
            "SFUK1091 - Home Décor, Gifts & Cards: 40% off all / 50% off when \
                spending over £40",
            "<br>",
            "Home Décor, Gifts & Cards: 40% off i all / 50% off when spending \
                over £40 Code - SFUK1091 31-05-2021",
            "<br>",
            "Programme Details",
            "<br>",
            "Sales",
            "Commission (per month) • 10%",
            "On All Sales",
            "• Cookie:- 7 Days • Product Feed Through AWIN • Voucher \
                Codes Here.",
            "• Read The Latest Blog Post About Snapfish • View Snapfish \
                Affiliate",
            "Programme Page • Or Check Out Other Clients Discount Codes Here",
            "<br>",
            "Optimus Performance",
            "Marketing Contact",
            "<br>",
            "Melissa Blue snapfish@optimus- pm.com +44 (0)1752 727852",
            "<br>",
            "About Optimus Performance Marketing Since 2006 Optimus \
                Performance Marketing has been at the forefront of \
                the affiliate marketing industry, winning numerous \
                industry awards and consistently maintaining \
                profitable growth.",
            "With experienced staff operating from our offices in Plymouth, \
                London and San Diego we deliver end to end management, \
                strategy and consultancy services worldwide to all sizes \
                of client and market vertical.",
            "We believe that success comes from continued evolution, so we \
                continually invest in training and tools to ensure that we \
                can offer class leading service while retaining staff \
                and clients.",
            "<br>",
            "We love to hear from anyone in the affiliate channel, so come \
                and find us at affiliate events or contact us at \
                info@optimus-pm.com.",
            "<br>",
            "© 2020 UK Affiliate Management Limited.",
            "No 05945086 Registered Office, Unit 22, Callywith Gate \
                Industrial Estate, Bodmin, Cornwall, PL31 2RQ.",
            "VAT Number:- 892 7678 56",
            "<br>",
            "•Optimus-pm gen3 MARKETING",
            "<br>",
            "You have received this email because you are a member of \
                Snapfish.co.uk affiliate program To unsubscribe simply visit \
                the link below.",
            "Click on link or paste into your browser \
                ui2.awin.com/unsubm.php?email=Aq9HfQtlwqzz35i%2F75%2B4\
                pjfbn6WG2Dqqbu9zLK7CEcw%3D&id=282949&merchant id=3850",
            "<br>",
            "VAVAV",
            "<br>",
        ];

        assert_eq!(expected_output, from_ocr(&document));
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
