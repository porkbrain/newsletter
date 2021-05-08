use shared::document::{Phrase, Source};
use std::{cmp::Ordering, fmt::Display};

/// Skip any voucher which has estimate lower than this.
const VOUCHER_SELECT_THRESHOLD: f64 = 0.8;

#[derive(Default, Debug, PartialEq)]
pub struct Voucher {
    pub phrase: String,
    pub text: String,
    pub estimate: f64,
    pub link: Option<String>,
    // useful for joining adjacent deals and vouchers
    pub(super) phrase_index: usize,
}

pub fn find_in(phrases: &[Phrase]) -> Vec<Voucher> {
    phrases
        .into_iter()
        .enumerate()
        .filter_map(|(pi, p)| {
            let e_d = *p.estimates.get(&Source::Dealc)?; // always there
            let w_d = 0.5;

            let vouchers: Vec<_> = p
                .words
                .iter()
                .filter_map(|w| {
                    let e_v = *w.estimates.get(&Source::Voucherc)?; // always there
                    let w_v = 0.8;

                    let e_c = w.estimates.get(&Source::CommonPhrases).copied();
                    let w_c = if e_c.is_some() { 1.0 } else { 0.0 };
                    let e_c = e_c.unwrap_or(0.0);

                    let e =
                        (e_d * w_d + e_v * w_v + e_c * w_c) / (w_d + w_v + w_c);

                    if e > VOUCHER_SELECT_THRESHOLD {
                        Some(Voucher::new(
                            pi,
                            p.text.clone(),
                            w.text.clone(),
                            e,
                        ))
                    } else {
                        None
                    }
                })
                .collect();

            if vouchers.is_empty() {
                None
            } else {
                Some(vouchers.into_iter())
            }
        })
        .flatten()
        .collect()
}

impl Voucher {
    pub fn new(
        phrase_index: usize,
        phrase_text: impl Display,
        text: impl Display,
        estimate: f64,
    ) -> Self {
        let phrase_text = phrase_text.to_string();
        let text = text.to_string();
        Self {
            phrase_index,
            text,
            estimate,
            phrase: phrase_text,
            ..Default::default()
        }
    }

    pub(super) fn cmp_estimates(&self, other: &Self) -> Ordering {
        self.estimate
            .partial_cmp(&other.estimate)
            .unwrap_or(Ordering::Equal)
    }
}

#[cfg(test)]
pub mod tests {
    use super::super::tests::testing_document;
    use super::*;

    #[test]
    fn it_should_select_vouchers() {
        let document = testing_document("default");

        assert_vouchers_approx_eq(
            find_in(document.phrases()),
            vec![
                Voucher::new(
                    0,
                    "Free Shipping on orders over £50 use code SHIPPING",
                    "SHIPPING",
                    0.892,
                ),
                Voucher::new(0, "Up to 30% OFF CODE ALPHA", "ALPHA", 0.966),
            ],
        )
    }

    pub fn assert_vouchers_approx_eq(
        actual: Vec<Voucher>,
        expected: Vec<Voucher>,
    ) {
        if expected.len() != actual.len() {
            println!("Expected: {:#?}", expected);
            println!("Actual: {:#?}", actual);
            panic!("Length mismatch!");
        }

        for (expected, actual) in expected.into_iter().zip(actual.into_iter()) {
            assert_eq!(expected.text, actual.text);
            assert_eq!(expected.phrase, actual.phrase);

            let diff = (expected.estimate - actual.estimate).abs();
            if diff > 0.03 {
                panic!(
                    "Estimates mismatch for '{}', expected {:.3} but
                    got {:.3}",
                    expected.text, expected.estimate, actual.estimate
                );
            }
        }
    }
}
