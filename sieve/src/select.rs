mod deal;
mod voucher;

use shared::document::Phrase;
use std::cmp::Ordering;

pub use deal::Deal;
pub use voucher::Voucher;

pub fn deals_and_vouchers(phrases: &[Phrase]) -> (Vec<Deal>, Vec<Voucher>) {
    let mut deals = deal::find_in(phrases);
    let mut vouchers = voucher::find_in(phrases);

    // remove deals which are already exported with vouchers
    deals.retain(|d| {
        for v in &mut vouchers {
            if d.text.contains(&v.phrase) {
                v.phrase = d.text.clone();
                v.estimate = v.estimate.max(d.estimate);
                return false;
            }
        }

        true
    });

    // deduplicate deals by text content
    deals.sort_by(|a, b| a.text.cmp(&b.text));
    deals.dedup_by(|a, b| a.text.eq_ignore_ascii_case(&b.text));

    // deduplicate vouchers but always retain the one with longer phrase
    vouchers.sort_by(|a, b| match a.text.cmp(&b.text) {
        Ordering::Equal => b.phrase.len().cmp(&a.phrase.len()),
        ord => ord,
    });
    vouchers.dedup_by(|a, b| a.text == b.text);

    // join adjacent deals and vouchers
    for v in &mut vouchers {
        deals.retain(|d| {
            // get the index of the first and last phrase the deal is formed
            // from
            //
            // if it's just one phrase the range will be of size 0 (that's the
            // unwrap there)
            let from_index = d.first_phrase_index;
            let to_index = d.last_phrase_index.unwrap_or(d.first_phrase_index);
            let deal_span_over_phrases = from_index..=to_index;

            if deal_span_over_phrases.contains(&v.phrase_index) {
                // steal the deal text and discard the deal
                v.phrase = d.text.clone();
                v.estimate = v.estimate.max(d.estimate);
                false
            } else if (to_index + 1) == v.phrase_index {
                // and we take + 1 because it can be adjacent, not only contained
                v.phrase = format!("{} {}", d.text, v.phrase);
                v.estimate = v.estimate.max(d.estimate);
                false
            } else {
                true
            }
        });
    }

    // and finally we pick only the top results
    deals.sort_by(|a, b| b.cmp_estimates(a));
    let deals = deals
        .into_iter()
        .enumerate()
        .take_while(|(di, d)| should_retain_offer(*di, d.estimate))
        .map(|(_, d)| d)
        .collect();

    vouchers.sort_by(|a, b| b.cmp_estimates(a));
    let vouchers = vouchers
        .into_iter()
        .enumerate()
        .take_while(|(vi, v)| should_retain_offer(*vi, v.estimate))
        .map(|(_, v)| v)
        .collect();

    (deals, vouchers)
}

fn should_retain_offer(ordinal: usize, estimate: f64) -> bool {
    const HARD_LIMIT: usize = 12;

    if ordinal > HARD_LIMIT {
        false
    } else {
        let ordinal = ordinal as f64;

        // curve which starts at .8 with first offer and goes slowly to 1.0,
        // so that by 5th offer it requires .949 estimate, and by 9th .992
        estimate >= 0.8 + (ordinal + 1.0).ln() / 12.0
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use deal::tests::assert_deals_approx_eq;
    use shared::Document;
    use std::fs;
    use voucher::tests::assert_vouchers_approx_eq;

    #[test]
    fn it_should_set_reasonable_thresholds_for_offers() {
        assert!(should_retain_offer(0, 1.0));
        assert!(!should_retain_offer(0, 0.5));
        assert!(should_retain_offer(0, 0.8));
        assert!(!should_retain_offer(1, 0.8));
        assert!(should_retain_offer(1, 1.0));
        assert!(should_retain_offer(1, 0.9));

        assert!(should_retain_offer(2, 1.0));
        assert!(should_retain_offer(3, 1.0));

        assert!(should_retain_offer(4, 1.0));
        assert!(!should_retain_offer(4, 0.9));

        assert!(should_retain_offer(8, 1.0));
        assert!(!should_retain_offer(8, 0.95));
        assert!(should_retain_offer(5, 0.95));
    }

    #[test]
    fn it_should_join_adjacent_deals_and_vouchers() {
        let document = testing_document("join_adjacent_deals_and_vouchers");

        let (_, vouchers) = deals_and_vouchers(document.phrases());

        assert_vouchers_approx_eq(
            vouchers,
            vec![
                Voucher::new(
                    0,
                    "Save 20% on Hair Repair Trial Kit! Use Code: 20OLAPLEX",
                    "20OLAPLEX",
                    1.0,
                ),
                Voucher::new(
                    0,
                    "Get an EXTRA 10% off selected brands at HQHair! \
                    Use Code: EXTRAX10",
                    "EXTRAX10",
                    1.0,
                ),
                Voucher::new(0, "Use Code: PAUL15", "PAUL15", 1.0),
            ],
        );
    }

    #[test]
    fn it_should_skip_duplicate_deals_ignore_case() {
        let document = testing_document("deduplicate_deals_ignore_case");

        let (deals, _) = deals_and_vouchers(document.phrases());

        assert_deals_approx_eq(
            deals,
            vec![
                Deal::new(
                    0,
                    "30th April - 2nd May - 25% off everything",
                    1.0
                ),
                Deal::new(
                    0,
                    "5th May - 7th May - Up to 50% off everything",
                    1.0,
                ),
                Deal::new(
                    0,
                    "LIVE TODAY - UP TO 40% OFF EVERYTHING",
                    1.0,
                ),
                Deal::new(
                    0,
                    "Subject: Dorothy Perkins - Live Today - Up to 40% off everything",
                    1.0,
                ),
                Deal::new(
                    0,
                    "3rd May - 4th May - 25% off everything + extra 10% off",
                    0.998,
                ),
            ]
        );
    }

    #[test]
    fn it_should_skip_duplicate_vouchers_and_keep_the_longer_phrased_one() {
        let document = testing_document("deduplicate_vouchers");

        let (deals, vouchers) = deals_and_vouchers(document.phrases());

        assert_vouchers_approx_eq(
            vouchers,
            vec![Voucher::new(0, "ALPHA slightly longer", "ALPHA", 1.0)],
        );

        assert_deals_approx_eq(deals, vec![]);
    }

    #[test]
    fn it_should_skip_duplicate_deals() {
        let document = testing_document("default");

        let (deals, vouchers) = deals_and_vouchers(document.phrases());

        assert_vouchers_approx_eq(
            vouchers,
            vec![
                Voucher::new(
                    0,
                    "Free Shipping on orders over £50 use code SHIPPING",
                    "SHIPPING",
                    0.986,
                ),
                Voucher::new(0, "Up to 30% OFF CODE ALPHA", "ALPHA", 0.958),
            ],
        );

        assert_eq!(deals, vec![]);
    }

    #[test]
    fn bug_skips_offers1() {
        let document = testing_document("bug_skips_offers1");

        let (deals, vouchers) = deals_and_vouchers(document.phrases());

        assert_vouchers_approx_eq(
            vouchers,
            vec![Voucher::new(
                0,
                "20% OFF the entire Beauty selection using promo \
                code MOTHERS21. Enter code in cart or simply \
                click on the link below.",
                "MOTHERS21",
                0.975,
            )],
        );
        assert_deals_approx_eq(
            deals,
            vec![
                Deal::new(
                    0,
                    "products from iHerb\'s entire Beauty selection \
                    and show her just",
                    0.981,
                ),
                Deal::new(0, "Use Promo Code:", 0.975),
            ],
        );
    }

    #[test]
    fn bug_skips_offers2() {
        let document = testing_document("bug_skips_offers2");

        let (_, vouchers) = deals_and_vouchers(document.phrases());

        assert_vouchers_approx_eq(
            vouchers,
            vec![
                Voucher::new(0, "Code: 300FF No MOV", "300FF", 1.0),
                Voucher::new(
                    0,
                    "Only valid on the selected range. The 15% discount will \
                    be applied at checkout after entering the code. Period of \
                    validity: until 09.05.2021 Code: SUMMER2K21AFF MOV: GBP 50",
                    "SUMMER2K21AFF",
                    1.0,
                ),
            ],
        );
    }

    #[test]
    fn bug_deduplicate_deals1() {
        let document = testing_document("bug_deduplicate_deals1");

        let (deals, _) = deals_and_vouchers(document.phrases());

        assert_deals_approx_eq(
            deals,
            vec![Deal::new(0, "Publisher: Discount Code - DailyMail", 0.939)],
        );
    }

    pub fn testing_document(name: &str) -> Document {
        let curr_path = fs::canonicalize(".").unwrap();

        let test_path = if curr_path.ends_with("sieve") {
            PathBuf::from("test/assets")
        } else if curr_path.ends_with("newsletter") {
            PathBuf::from("sieve/test/assets")
        } else {
            panic!("Test must be called from /newsletter or /newsletter/sieve")
        }
        .join(format!("{}.json", name));

        let contents =
            fs::read_to_string(&test_path).expect("Cannot read test file");

        serde_json::from_str(&contents).unwrap()
    }
}
