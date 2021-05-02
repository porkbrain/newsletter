const VOUCHER_KEYWORDS: &[&str] = &["voucher", "code", "discount", "coupon"];

const COMMON_PHRASES: &[&[Token]] = &[
    &[
        Token::Term(&["use", "using", "redeem", "apply", "enter", "insert"]),
        Token::Any(4),
        Token::Voucher,
        Token::Any(7),
    ],
    &[
        Token::Term(&["your"]),
        Token::Any(1),
        Token::Voucher,
        Token::Any(3),
    ],
    &[
        Token::Term(&["get", "shop"]),
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
struct CommonPhrase {
    grammar: Vec<Token>,
    constituents: Vec<(usize, WordKind)>,
    is_done: bool,
}

pub fn word_estimates(words: &[&str]) -> Option<Vec<f64>> {
    let mut matches: Vec<CommonPhrase> = vec![];

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
            let start_new = match grammar[0] {
                Token::Any(_) | Token::Voucher => {
                    panic!("Grammar must start with Token::Term")
                }
                Token::Term(t) => in_term(word, t),
            };

            if start_new {
                matches.push(CommonPhrase {
                    grammar: grammar.iter().skip(1).cloned().collect(),
                    constituents: vec![(wi, WordKind::Phrasal)],
                    is_done: false,
                })
            }
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
    term.iter().any(|t| w.contains(t))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_word_estimates() {
        assert_eq!(
            word_estimates(&[
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
            word_estimates(&[
                "Discount", "code", "is", "valid", "until", "midnight",
                "Monday", "30th", "November", "2020",
            ]),
            None
        );

        assert_eq!(
            word_estimates(&[
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
            word_estimates(&["use", "code", "ALPHA"]).unwrap(),
            &[0.0, 0.0, 1.0]
        );

        assert_eq!(
            word_estimates(&["use", "this", "code", "ALPHA"]).unwrap(),
            &[0.0, 0.85, 0.0, 1.0]
        );
    }
}
