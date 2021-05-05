use std::{cmp::Ordering, fmt::Display};

use shared::phrases::{Phrase, Source};

/// Skip any phrase which has estimate lower than this.
const DEAL_SELECT_THRESHOLD: f64 = 0.8;

/// How close in estimates must two adjacent phrases be to merge them into one
/// deal.
const MERGE_DEALS_THRESHOLD: f64 = 0.05;

/// Skip any voucher which has estimate lower than this.
const VOUCHER_SELECT_THRESHOLD: f64 = 0.8;

// TODO:
const LIMIT_VOUCHERS: usize = 5;
const LIMIT_DEALS: usize = 5;

#[derive(Default, Debug, PartialEq)]
pub struct Deal {
    pub text: String,
    pub estimate: f64,
    pub link: Option<String>,
    // useful for joining adjacent deals and vouchers
    first_phrase_index: usize,
    // if multiple adjacent phrases were merged to create this one, this should
    // be the index of last one
    last_phrase_index: Option<usize>,
}

#[derive(Default, Debug, PartialEq)]
pub struct Voucher {
    pub phrase: String,
    pub text: String,
    pub estimate: f64,
    pub link: Option<String>,
    // useful for joining adjacent deals and vouchers
    first_phrase_index: usize,
}

pub fn deals_and_vouchers(phrases: &[Phrase]) -> (Vec<Deal>, Vec<Voucher>) {
    let mut deals = deals(phrases);
    let mut vouchers = vouchers(phrases);

    // remove deals which are already exported with vouchers
    deals.retain(|d| {
        for voucher in &vouchers {
            if d.text == voucher.phrase {
                return false;
            }
        }

        true
    });

    // deduplicate deals by text content
    deals.sort_by(|a, b| a.text.cmp(&b.text));
    deals.dedup_by(|a, b| a.text.eq_ignore_ascii_case(&b.text));

    deals.sort_by(|a, b| b.cmp_estimates(a));
    deals.truncate(LIMIT_DEALS);

    // deduplicate vouchers but always retain the one with longer phrase
    vouchers.sort_by(|a, b| match a.text.cmp(&b.text) {
        Ordering::Equal => b.phrase.len().cmp(&a.phrase.len()),
        ord => ord,
    });
    vouchers.dedup_by(|a, b| a.text == b.text);

    // now we can pick only
    vouchers.sort_by(|a, b| b.cmp_estimates(a));
    vouchers.truncate(LIMIT_VOUCHERS);

    (deals, vouchers)
}

fn deals(phrases: &[Phrase]) -> Vec<Deal> {
    // calculates estimates for each phrase based on estimate each method
    // contributes scaled down by its relevant weight, tweaked for the task
    let mut estimates = phrases
        .into_iter()
        .map(|p| {
            // special phrase which denotes new paragraph, useful for the step
            // after this one where we join adjacent deals together
            if p.text == "<br>" {
                return Some(0.0);
            }

            let top_w = p.top_word()?;

            // dealc is the most relevant
            let e_d = *p.estimates.get(&Source::Dealc)?; // always there
            let w_d = 1.0;

            // voucherc is not that interesting for deals, because sometimes it
            // identifies weird words, and not all deals have a voucher
            let e_v = *top_w.estimates.get(&Source::Voucherc)?; // always there
            let w_v = 0.1;

            // similarly, common phrases are not that important either, but they
            // tend to have lower false positive rate to voucherc
            let e_c = top_w.estimates.get(&Source::CommonPhrases).copied();
            let w_c = if e_c.is_some() { 0.2 } else { 0.0 };
            let e_c = e_c.unwrap_or(0.0);

            Some((e_d * w_d + e_v * w_v + e_c * w_c) / (w_d + w_v + w_c))
        })
        .enumerate();

    // let's merge adjacent deals into one
    let mut deals: Vec<Deal> = vec![];
    let mut cdeal: Option<Deal> = None;
    while let Some((pi, cpe)) = estimates.next() {
        // cde = current deal estimate
        // cpe = currently iterated phrase estimate
        let cde = cdeal.as_ref().map(|d| d.estimate);
        match (cde, cpe) {
            (None, None) => (),
            (Some(_), None) => {
                cdeal.take().map(|d| deals.push(d));
            }
            (Some(cde), Some(cpe)) => {
                // if they are of similar estimates, merge them
                if (cde - cpe).abs() < MERGE_DEALS_THRESHOLD {
                    cdeal = cdeal.take().map(|d| {
                        d.merge(&Deal::new(pi, &phrases[pi].text, cpe))
                    });
                } else {
                    // they differ in estimate too much, separate them out
                    cdeal.take().map(|d| deals.push(d));
                    if cpe > DEAL_SELECT_THRESHOLD {
                        cdeal = Some(Deal::new(pi, &phrases[pi].text, cpe));
                    }
                }
            }
            (None, Some(cpe)) => {
                // there was no deal to append it to, start a new one
                if cpe > DEAL_SELECT_THRESHOLD {
                    cdeal = Some(Deal::new(pi, &phrases[pi].text, cpe));
                }
            }
        }
    }

    deals
}

fn vouchers(phrases: &[Phrase]) -> Vec<Voucher> {
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

impl Deal {
    pub fn new(phrase_index: usize, text: impl Display, estimate: f64) -> Self {
        let text = text.to_string();
        Self {
            first_phrase_index: phrase_index,
            text,
            estimate,
            ..Default::default()
        }
    }

    fn merge(mut self, other: &Self) -> Self {
        self.text.push(' ');
        self.text.push_str(&other.text);
        self.estimate = self.estimate.max(other.estimate);
        self
    }

    fn cmp_estimates(&self, other: &Self) -> Ordering {
        self.estimate
            .partial_cmp(&other.estimate)
            .unwrap_or(Ordering::Equal)
    }
}

impl Voucher {
    pub fn new(
        first_phrase_index: usize,
        phrase_text: impl Display,
        text: impl Display,
        estimate: f64,
    ) -> Self {
        let phrase_text = phrase_text.to_string();
        let text = text.to_string();
        Self {
            first_phrase_index,
            text,
            estimate,
            phrase: phrase_text,
            ..Default::default()
        }
    }

    fn cmp_estimates(&self, other: &Self) -> Ordering {
        self.estimate
            .partial_cmp(&other.estimate)
            .unwrap_or(Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::Phrases;

    #[test]
    fn it_should_join_adjacent_deals() {
        let phrases = testing_document("join_adjacent_deals");

        assert_deals_approx_eq(
            deals(phrases.inner()),
            vec![
                Deal::new(0, "Enjoy 50% off everything", 0.809),
                Deal::new(
                    0,
                    "50% off all collections offer cannot be used in",
                    0.0,
                )
                .merge(&Deal::new(
                    0,
                    "conjunction with any other discount or offer.",
                    0.0,
                ))
                .merge(&Deal::new(
                    0,
                    "50% off excludes delivery if applicable. We reserve the",
                    0.0,
                ))
                .merge(&Deal::new(
                    0,
                    "right to cease the 50% promotion at any time",
                    0.915,
                )),
            ],
        );
    }

    #[test]
    fn it_should_skip_duplicate_deals_ignore_case() {
        let phrases = testing_document("deduplicate_deals_ignore_case");

        let (deals, _) = deals_and_vouchers(phrases.inner());

        assert_deals_approx_eq(
            deals,
            vec![
                Deal::new(
                    0,
                    "LIVE TODAY - UP TO 40% OFF EVERYTHING",
                    0.957,
                ),
                Deal::new(
                    0,
                    "30th April - 2nd May - 25% off everything",
                    0.915
                ),
                Deal::new(
                    0,
                    "5th May - 7th May - Up to 50% off everything",
                    0.915,
                ),
                Deal::new(
                    0,
                    "3rd May - 4th May - 25% off everything + extra 10% off",
                    0.913,
                ),
                Deal::new(
                    0,
                    "Subject: Dorothy Perkins - Live Today - Up to 40% off everything",
                    0.910,
                ),
            ]
        );
    }

    #[test]
    fn it_should_skip_duplicate_vouchers_and_keep_the_longer_phrased_one() {
        let phrases: Phrases = testing_document("deduplicate_vouchers");

        let (deals, vouchers) = deals_and_vouchers(phrases.inner());

        assert_vouchers_approx_eq(
            vouchers,
            vec![Voucher::new(0, "ALPHA slightly longer", "ALPHA", 1.0)],
        );

        assert_deals_approx_eq(deals, vec![]);
    }

    #[test]
    fn it_should_skip_duplicate_deals() {
        let phrases = testing_document("default");

        let (deals, vouchers) = deals_and_vouchers(phrases.inner());

        assert_vouchers_approx_eq(
            vouchers,
            vec![
                Voucher::new(0, "Up to 30% OFF CODE ALPHA", "ALPHA", 0.958),
                Voucher::new(
                    0,
                    "Free Shipping on orders over £50 use code SHIPPING",
                    "SHIPPING",
                    0.892,
                ),
            ],
        );

        assert_eq!(deals, vec![]);
    }

    #[test]
    fn it_should_select_deals() {
        let phrases = testing_document("default");

        assert_deals_approx_eq(
            deals(phrases.inner()),
            vec![
                Deal::new(
                    0,
                    "Free Shipping on orders over £50 use code SHIPPING",
                    0.963,
                ),
                Deal::new(0, "Up to 30% OFF CODE ALPHA", 0.9677482825634689),
            ],
        )
    }

    #[test]
    fn it_should_select_vouchers() {
        let phrases = testing_document("default");

        assert_vouchers_approx_eq(
            vouchers(phrases.inner()),
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

    fn assert_deals_approx_eq(expected: Vec<Deal>, actual: Vec<Deal>) {
        if expected.len() != actual.len() {
            println!("Expected: {:#?}", expected);
            println!("Actual: {:#?}", actual);
            panic!("Length mismatch!");
        }

        for (expected, actual) in expected.into_iter().zip(actual.into_iter()) {
            assert_eq!(expected.text, actual.text);

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

    fn assert_vouchers_approx_eq(expected: Vec<Voucher>, actual: Vec<Voucher>) {
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

    fn testing_document(name: &str) -> Phrases {
        let contents = match name {
            "default" => include_str!("../test/assets/default.json"),
            "deduplicate_deals_ignore_case" => include_str!(
                "../test/assets/deduplicate_deals_ignore_case.json"
            ),
            "deduplicate_vouchers" => {
                include_str!("../test/assets/deduplicate_vouchers.json")
            }
            "join_adjacent_deals" => {
                include_str!("../test/assets/join_adjacent_deals.json")
            }
            _ => panic!("No such testing file"),
        };

        serde_json::from_str(contents).unwrap()
    }
}
