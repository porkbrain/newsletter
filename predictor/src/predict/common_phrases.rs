use shared::phrases::Word;

const VOUCHER_KEYWORDS: &[&str] = &["voucher", "code", "discount", "coupon"];

const COMMON_PHRASES: &[&[Token]] = &[
    &[
        Token::Term(&[
            "use", "used", "using", "redeem*", "apply*", "enter*", "offer*",
            "insert*",
        ]),
        Token::Any(4),
        Token::Voucher,
        Token::Any(7),
    ],
    &[
        Token::Term(&["your", "with"]),
        Token::Any(1),
        Token::Voucher,
        Token::Any(3),
    ],
    &[
        Token::Term(&["get*", "shop*"]),
        Token::Any(2),
        Token::Term(&["with"]),
        Token::Any(3),
        Token::Voucher,
        Token::Any(4),
    ],
];

#[derive(Debug, Clone)]
enum Token {
    Term(&'static [&'static str]),
    Any(usize),
    Voucher,
}

#[derive(Debug, Clone)]
enum WordKind {
    Candidate,
    Voucher,
    Phrasal,
}

#[derive(Debug)]
struct CommonPhraseTracker {
    grammar: Vec<Token>,
    constituents: Vec<(usize, WordKind)>,
    is_done: bool,
}

const ESTIMATE_FOR_NOT_MATCHED_WORDS: f64 = 0.25;

pub fn word_estimates(words: &[&Word]) -> Option<Vec<f64>> {
    let a = over_special_chars(words);
    let b = over_long_phrases(words.iter().map(|w| w.text.as_str()));

    match (a, b) {
        (None, o) => o,
        (o, None) => o,
        (Some(esa), Some(esb)) => {
            Some(
                esa.into_iter()
                    .zip(esb.into_iter())
                    .map(|(ea, eb)| {
                        // if it's been given estimate of zero, then we know it's a phrasal
                        // word
                        let keep_zero = ea == 0.0 || eb == 0.0;
                        if keep_zero {
                            0.0
                        } else {
                            ea.max(eb)
                        }
                    })
                    .collect(),
            )
        }
    }
}

fn over_special_chars(words: &[&Word]) -> Option<Vec<f64>> {
    let mut estimates: Vec<_> = (0..words.len())
        .map(|_| ESTIMATE_FOR_NOT_MATCHED_WORDS)
        .collect();

    let mut any_matched = false;
    for (wi, w) in words.iter().enumerate() {
        if is_voucher_keyword(&w.text.to_lowercase()) {
            if w.raw.ends_with(':') {
                if let Some(w) = estimates.get_mut(wi + 1) {
                    *w = 1.0;
                    any_matched = true;
                    *estimates.get_mut(wi).unwrap() = 0.0;
                }
            }
        }
    }

    if any_matched {
        Some(estimates)
    } else {
        None
    }
}

fn over_long_phrases<'a>(
    words: impl IntoIterator<Item = &'a str>,
) -> Option<Vec<f64>> {
    let words: Vec<_> = words.into_iter().map(|w| w.to_lowercase()).collect();
    let mut matches: Vec<CommonPhraseTracker> = vec![];

    for (wi, word) in words.iter().enumerate() {
        matches = matches
            .into_iter()
            .filter_map(|mut m| {
                if m.is_done {
                    return Some(m);
                }

                let (current_token, next_token) = {
                    let mut view = m.grammar.iter();
                    (view.next().cloned(), view.next().cloned())
                };

                let mut remove_one_token = || drop(m.grammar.remove(0));
                match current_token {
                    None => m.is_done = true,
                    Some(Token::Term(t)) if in_term(word, t) => {
                        remove_one_token();
                        m.constituents.push((wi, WordKind::Phrasal));
                    }
                    Some(Token::Voucher) if in_term(word, VOUCHER_KEYWORDS) => {
                        remove_one_token();
                        m.constituents.push((wi, WordKind::Voucher));
                    }
                    Some(Token::Any(0)) if next_token.is_none() => {
                        remove_one_token()
                    }
                    Some(Token::Any(0)) | Some(Token::Voucher)
                    | Some(Token::Term(_)) => return None,
                    Some(Token::Any(n)) => {
                        match next_token {
                            Some(Token::Term(t)) if in_term(word, t) => {
                                remove_one_token();
                                remove_one_token();
                                m.constituents.push((wi, WordKind::Phrasal));
                            }
                            Some(Token::Voucher)
                                if is_voucher_keyword(word) =>
                            {
                                remove_one_token();
                                remove_one_token();
                                m.constituents.push((wi, WordKind::Voucher));
                            }
                            _ => {
                                match (next_token, words.get(wi + 1)) {
                                    (Some(Token::Term(t)), Some(next_word))
                                        if in_term(next_word, t) =>
                                    {
                                        remove_one_token();
                                    }
                                    (Some(Token::Voucher), Some(next_word))
                                        if is_voucher_keyword(next_word) =>
                                    {
                                        remove_one_token();
                                    }
                                    _ => m.grammar[0] = Token::Any(n - 1),
                                };
                                m.constituents.push((wi, WordKind::Candidate));
                            }
                        };
                    }
                };

                Some(m)
            })
            .collect();

        // start new matches based on the current word
        for grammar in COMMON_PHRASES {
            debug_assert!(grammar.len() > 0);
            match grammar[0] {
                Token::Any(_) => {
                    panic!("Grammar mustn't start with Token::Any")
                }
                Token::Voucher if is_voucher_keyword(word) => {
                    matches.push(CommonPhraseTracker {
                        grammar: grammar.iter().skip(1).cloned().collect(),
                        constituents: vec![(wi, WordKind::Voucher)],
                        is_done: false,
                    })
                }
                Token::Term(t) if in_term(word, t) => {
                    matches.push(CommonPhraseTracker {
                        grammar: grammar.iter().skip(1).cloned().collect(),
                        constituents: vec![(wi, WordKind::Phrasal)],
                        is_done: false,
                    })
                }
                _ => (),
            };
        }
    }

    let mut estimates_for_words = matches
        .into_iter()
        .filter_map(|m| {
            let (current_token, next_token) = {
                let mut view = m.grammar.iter();
                (view.next().cloned(), view.next().cloned())
            };

            match current_token {
                None if m.is_done => Some(m.constituents),
                Some(Token::Any(_)) if next_token.is_none() => {
                    Some(m.constituents)
                }
                _ => None,
            }
        })
        .map(|c| {
            // FIXME: is buggy when grammar has more than one Token::Voucher
            let voucher_keyword_pos = *c
                .iter()
                .find(|(_, kind)| matches!(kind, &WordKind::Voucher))
                .map(|(wi, _)| wi)
                .expect("Each grammar must contain a Token::Voucher");

            c.into_iter()
                .map(|(wi, kind)| match kind {
                    WordKind::Voucher | WordKind::Phrasal => (wi, 0.0),
                    WordKind::Candidate => (
                        wi,
                        estimate_from_dist_to_voucher_keyword(
                            wi as isize - voucher_keyword_pos as isize,
                        ),
                    ),
                })
                .collect::<Vec<(usize, f64)>>()
        })
        .flatten()
        .peekable();

    // give estimates only if at least one common phrase was found
    if estimates_for_words.peek().is_some() {
        let mut estimates: Vec<_> = (0..words.len()).map(|_| 0.25).collect();

        for (wi, estimate) in estimates_for_words {
            // if it's been given estimate of zero, then we know it's a phrasal
            // word
            let keep_zero = estimates[wi] == 0.0 || estimate == 0.0;
            estimates[wi] = if keep_zero {
                0.0
            } else {
                estimate.max(estimates[wi])
            };
        }

        Some(estimates)
    } else {
        None
    }
}

fn estimate_from_dist_to_voucher_keyword(dist: isize) -> f64 {
    let is_positioned_after = dist.is_positive();
    let dist = (dist as f64).abs().max(1.0);

    // is higher with smaller dist
    let coef = 0.3;
    let inverse_root_bonus = 1.0 / dist.powf(coef);

    // english is left to right
    let direction_penalty = if is_positioned_after { 0.0 } else { 0.15 };

    (1.0 + inverse_root_bonus) / 2.0 - direction_penalty
}

fn is_voucher_keyword(w: &str) -> bool {
    in_term(w, VOUCHER_KEYWORDS)
}

fn in_term(w: &str, term: &[&str]) -> bool {
    term.iter().any(|t| {
        if t.ends_with('*') {
            w.contains(&t[..t.len() - 2])
        } else {
            t == &w
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gives_over_long_phrases() {
        assert_eq!(
            over_long_phrases(vec![
                "To", "use", "simply", "enter", "the", "code", "CYBER26", "in",
                "the", "basket", "and", "click", "go",
            ])
            .unwrap(),
            &[
                0.25,
                0.0,
                0.7096115466624322,
                0.0,
                0.85,
                0.0,
                1.0,
                0.9061261981781177,
                0.8596115466624322,
                0.8298769776932235,
                0.8085169313600048,
                0.7920953405339328,
                0.778894912651623
            ]
        );

        assert_eq!(
            over_long_phrases(vec![
                "Discount", "code", "is", "valid", "until", "midnight",
                "Monday", "30th", "November", "2020",
            ]),
            None
        );

        assert_eq!(
            over_long_phrases(vec![
                "Discount",
                "codes",
                "availability",
                "and",
                "can",
                "discount",
                "may",
                "be",
                "used",
                "per",
                "order",
                "and",
                "these",
            ]),
            None
        );
    }

    #[test]
    fn it_can_find_voucher_close_to_phrase() {
        assert_eq!(
            over_long_phrases(vec!["use", "code", "ALPHA"]).unwrap(),
            &[0.0, 0.0, 1.0]
        );

        assert_eq!(
            over_long_phrases(vec!["use", "this", "code", "ALPHA"]).unwrap(),
            &[0.0, 0.85, 0.0, 1.0]
        );
    }

    #[test]
    fn it_is_case_insensitive() {
        assert_eq!(
            over_long_phrases(vec!["redeEM", "coupon", "ALPHA"]).unwrap(),
            &[0.0, 0.0, 1.0]
        );
    }

    #[test]
    fn it_works_with_star_matching() {
        assert_eq!(
            over_long_phrases(vec!["redeeming", "coupon", "ALPHA"]).unwrap(),
            &[0.0, 0.0, 1.0]
        );
        assert_eq!(
            over_long_phrases(vec!["username", "ukdiscount", "ALPHA"]),
            None
        );
    }

    #[test]
    fn it_gives_estimates_over_short_range_and_long_range() {
        let word_estimates_from_list = |list: Vec<_>| {
            word_estimates(list.iter().collect::<Vec<_>>().as_slice())
        };

        let words =
            vec![word_from("use"), word_from("code"), word_from("ALPHA")];
        assert_eq!(word_estimates_from_list(words), Some(vec![0.0, 0.0, 1.0]));

        let words = vec![
            word_from("blabla"),
            Word::new_with_raw("Code".to_string(), "Code:".to_string()),
            word_from("ALPHA"),
        ];
        assert_eq!(word_estimates_from_list(words), Some(vec![0.25, 0.0, 1.0]));

        let words = vec![
            word_from("this"),
            word_from("is"),
            word_from("long"),
            word_from("use"),
            word_from("code"),
            word_from("ALPHA"),
            word_from("haha"),
            word_from("its 3AM"),
        ];
        assert_eq!(
            word_estimates_from_list(words),
            Some(vec![
                0.25,
                0.25,
                0.25,
                0.0,
                0.0,
                1.0,
                0.9061261981781177,
                0.8596115466624322
            ])
        );

        let words = vec![
            word_from("use"),
            Word::new_with_raw("Code".to_string(), "Code:".to_string()),
            word_from("ALPHA"),
        ];
        assert_eq!(word_estimates_from_list(words), Some(vec![0.0, 0.0, 1.0]));
    }

    fn word_from(s: &str) -> Word {
        Word {
            raw: s.to_string(),
            text: s.to_string(),
            estimates: Default::default(),
        }
    }
}
