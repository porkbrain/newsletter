use shared::phrases::{Phrase, Source};
use std::{cmp::Ordering, fmt::Display};

/// Skip any phrase which has estimate lower than this.
const DEAL_SELECT_THRESHOLD: f64 = 0.8;

#[derive(Default, Debug, PartialEq)]
pub struct Deal {
    pub text: String,
    pub estimate: f64,
    pub link: Option<String>,
    // useful for joining adjacent deals and vouchers
    pub(super) first_phrase_index: usize,
    // if multiple adjacent phrases were merged to create this one, this should
    // be the index of last one
    pub(super) last_phrase_index: Option<usize>,
}

pub fn find_in(phrases: &[Phrase]) -> Vec<Deal> {
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

            // common phrases are useful only if they match, but they don't tell
            // us much when they don't match, that's why we've got a limit there
            let e_c = top_w.estimates.get(&Source::CommonPhrases).copied();
            let w_c = if e_c.filter(|e_c| *e_c > 0.8).is_some() {
                0.2
            } else {
                0.0
            };
            let e_c = e_c.unwrap_or(0.0);

            Some((e_d * w_d + e_c * w_c) / (w_d + w_c))
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
                // if they are of similar estimates or both very high, merge them
                if should_be_merged(cde, cpe) {
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

    pub(super) fn cmp_estimates(&self, other: &Self) -> Ordering {
        self.estimate
            .partial_cmp(&other.estimate)
            .unwrap_or(Ordering::Equal)
    }
}

fn should_be_merged(e1: f64, e2: f64) -> bool {
    // The higher the estimate, the more favorable the merging is.
    // The function below looks something like this:
    //
    // ```
    //   1
    //   |
    //  ...
    //   |
    //   |                    _  0.15
    //   |                  ./
    //   |                ./
    //   |               ..
    // 0 +---------------|-----+
    //   0        0.5          1
    // ```
    //
    // Where x axis is the larger of the two estimates, and y axis is the
    // threshold for for the difference between them so that they are merged.
    //
    // https://www.desmos.com/calculator/7ydyy9s6ze
    let threshold = 0.15 - 3.0 * (1.0 - e1.max(e2)).powi(2);
    (e1 - e2).abs() < threshold
}

#[cfg(test)]
pub mod tests {
    use super::super::tests::testing_document;
    use super::*;

    #[test]
    fn it_sets_reasonable_merge_thresholds() {
        assert!(!should_be_merged(0.8, 0.2));
        assert!(!should_be_merged(0.2, 0.8));
        assert!(should_be_merged(0.78, 0.8));
        assert!(should_be_merged(0.85, 0.9));
        assert!(!should_be_merged(0.75, 1.0));
        assert!(should_be_merged(0.89, 1.0));
    }

    #[test]
    fn it_should_join_adjacent_deals() {
        let phrases = testing_document("join_adjacent_deals");

        assert_deals_approx_eq(
            find_in(phrases.inner()),
            vec![
                Deal::new(0, "50% off everything", 0.94),
                Deal::new(0, "30 Apr 2021 Excellent service", 0.85),
                Deal::new(0, "Enjoy 50% off everything", 1.0),
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
                    1.0,
                )),
            ],
        );
    }

    #[test]
    fn it_should_select_deals() {
        let phrases = testing_document("default");

        assert_deals_approx_eq(
            find_in(phrases.inner()),
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

    pub fn assert_deals_approx_eq(actual: Vec<Deal>, expected: Vec<Deal>) {
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
}
