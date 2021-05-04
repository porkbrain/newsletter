use std::cmp::Ordering;

use shared::phrases::{Phrase, Source};

const DEAL_SELECT_THRESHOLD: f64 = 0.8;
const LIMIT_DEALS: usize = 5;

#[derive(Default, Debug, PartialEq)]
pub struct Deal {
    pub text: String,
    pub estimate: f64,
    pub link: Option<String>,
}

#[derive(Default, Debug, PartialEq)]
pub struct Voucher {
    pub phrase: String,
    pub text: String,
    pub estimate: f64,
    pub link: Option<String>,
}

pub fn deals_and_vouchers(phrases: &[Phrase]) -> (Vec<Deal>, Vec<Voucher>) {
    let mut deals = deals(phrases);
    let vouchers = vouchers(phrases);

    // remove deals which are already exported with vouchers
    deals.retain(|d| {
        for voucher in &vouchers {
            if d.text == voucher.phrase {
                return false;
            }
        }

        true
    });

    (deals, vouchers)
}

fn deals(phrases: &[Phrase]) -> Vec<Deal> {
    let mut deals: Vec<_> = phrases
        .into_iter()
        .filter_map(|p| {
            let top_w = p.top_word()?;

            let e_d = *p.estimates.get(&Source::Dealc)?; // always there
            let w_d = 1.0;

            let e_v = *top_w.estimates.get(&Source::Voucherc)?; // always there
            let w_v = 0.1;

            let e_c = top_w.estimates.get(&Source::CommonPhrases).copied();
            let w_c = if e_c.is_some() { 0.25 } else { 0.0 };
            let e_c = e_c.unwrap_or(0.0);

            let e = (e_d * w_d + e_v * w_v + e_c * w_c) / (w_d + w_v + w_c);

            if e > DEAL_SELECT_THRESHOLD {
                Some(Deal::new(p.text.clone(), e))
            } else {
                None
            }
        })
        .collect();

    deals.sort_by(|a, b| b.cmp(a));
    deals.truncate(LIMIT_DEALS);

    deals
}

const VOUCHER_SELECT_THRESHOLD: f64 = 0.8;
const LIMIT_VOUCHERS: usize = 5;

fn vouchers(phrases: &[Phrase]) -> Vec<Voucher> {
    let mut vouchers: Vec<_> = phrases
        .into_iter()
        .filter_map(|p| {
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
                        Some(Voucher::new(p.text.clone(), w.text.clone(), e))
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
        .collect();

    vouchers.sort_by(|a, b| b.cmp(a));

    vouchers.truncate(LIMIT_VOUCHERS);
    vouchers
}

impl Deal {
    pub fn new(text: String, estimate: f64) -> Self {
        Self {
            text,
            estimate,
            ..Default::default()
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        self.estimate
            .partial_cmp(&other.estimate)
            .unwrap_or(Ordering::Equal)
    }
}

impl Voucher {
    pub fn new(phrase: String, text: String, estimate: f64) -> Self {
        Self {
            text,
            estimate,
            phrase,
            ..Default::default()
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        self.estimate
            .partial_cmp(&other.estimate)
            .unwrap_or(Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use shared::Phrases;

    #[test]
    fn it_should_skip_duplicate_deals() {
        let phrases = testing_document();

        let (deals, vouchers) = deals_and_vouchers(phrases.inner());

        assert_eq!(
            vouchers,
            vec![
                Voucher::new(
                    "Up to 30% OFF CODE ALPHA".to_string(),
                    "ALPHA".to_string(),
                    0.9585623288901614
                ),
                Voucher::new(
                    "Free Shipping on orders over £50 use code SHIPPING"
                        .to_string(),
                    "SHIPPING".to_string(),
                    0.8922419680443364
                )
            ]
        );

        assert_eq!(deals, vec![]);
    }

    #[test]
    fn it_should_select_deals() {
        let phrases = testing_document();

        assert_eq!(
            deals(phrases.inner()),
            vec![
                Deal::new(
                    "Up to 30% OFF CODE ALPHA".to_string(),
                    0.9677482825634689
                ),
                Deal::new(
                    "Free Shipping on orders over £50 use code SHIPPING"
                        .to_string(),
                    0.964979770972405,
                )
            ]
        )
    }

    #[test]
    fn it_should_select_vouchers() {
        let phrases = testing_document();

        assert_eq!(
            vouchers(phrases.inner()),
            vec![
                Voucher::new(
                    "Up to 30% OFF CODE ALPHA".to_string(),
                    "ALPHA".to_string(),
                    0.9585623288901614
                ),
                Voucher::new(
                    "Free Shipping on orders over £50 use code SHIPPING"
                        .to_string(),
                    "SHIPPING".to_string(),
                    0.8922419680443364
                )
            ]
        )
    }

    fn testing_document() -> Phrases {
        let json = json!([
            {
                "text": "View Online Men Women Kids",
                "estimates": {
                    "dealc": 0.024
                },
                "words": [
                    {
                        "text": "View",
                        "estimates": {
                            "voucherc": 0.01020154356956482,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "Online",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.014664024114608765
                        }
                    },
                    {
                        "text": "Men",
                        "estimates": {
                            "voucherc": 0.0006843507289886475,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "Women",
                        "estimates": {
                            "voucherc": 0.017828315496444702,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "Kids",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.01020154356956482
                        }
                    }
                ]
            },
            {
                "text": "Free Shipping on orders over £50 use code SHIPPING",
                "estimates": {
                    "dealc": 0.9826166666666668
                },
                "words": [
                    {
                        "text": "Free",
                        "estimates": {
                            "voucherc": 0.01020154356956482,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "Shipping",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.007294446229934692
                        }
                    },
                    {
                        "text": "orders",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.012203812599182127
                        }
                    },
                    {
                        "text": "over",
                        "estimates": {
                            "voucherc": 0.02872207760810852,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "£50",
                        "estimates": {
                            "voucherc": 0.00011351741704856976,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "use",
                        "estimates": {
                            "voucherc": 0.01020154356956482,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "code",
                        "estimates": {
                            "voucherc": 0.010602414608001707,
                            "open_ai": 0.4,
                            "common_phrases": 0.0,
                        }
                    },
                    {
                        "text": "SHIPPING",
                        "estimates": {
                            "voucherc": 0.7010602414608001707,
                            "open_ai": 0.4,
                            "common_phrases": 1,
                        }
                    }
                ]
            },
            {
                "text": "Up to 30% OFF CODE ALPHA",
                "estimates": {
                    "dealc": 0.9662650793650792
                },
                "words": [
                    {
                        "text": "30%",
                        "estimates": {
                            "voucherc": 0.0003882348537445069,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "OFF",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.30195102095603943
                        }
                    },
                    {
                        "text": "CODE",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.30195102095603943,
                            "common_phrases": 0.0,
                        }
                    },
                    {
                        "text": "ALPHA",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.90195102095603943,
                            "common_phrases": 1.0,
                        }
                    }
                ]
            },
            {
                "text": "They won't be around forever",
                "estimates": {
                    "dealc": 0.000497765646820672
                },
                "words": [
                    {
                        "text": "They",
                        "estimates": {
                            "voucherc": 0.01020154356956482,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "won't",
                        "estimates": {
                            "voucherc": 0.004802733659744263,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "around",
                        "estimates": {
                            "voucherc": 0.012203812599182127,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "forever",
                        "estimates": {
                            "voucherc": 0.01134440302848816,
                            "open_ai": 0.4
                        }
                    }
                ]
            },
            {
                "text": "Up to 30% off",
                "estimates": {
                    "dealc": 0.3662650793650792
                },
                "words": [
                    {
                        "text": "30%",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.0003882348537445069
                        }
                    },
                    {
                        "text": "off",
                        "estimates": {
                            "voucherc": 0.0020304322242736816,
                            "open_ai": 0.4
                        }
                    }
                ]
            },
            {
                "text": "SHOP NOW 574 > 680 > 373 > 500 >",
                "estimates": {
                    "dealc": 0.009256287044126343
                },
                "words": [
                    {
                        "text": "SHOP",
                        "estimates": {
                            "voucherc": 0.3672136068344116,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "NOW",
                        "estimates": {
                            "voucherc": 0.30195102095603943,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "574",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.028750747442245483
                        }
                    },
                    {
                        "text": "680",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.028750747442245483
                        }
                    },
                    {
                        "text": "373",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.028750747442245483
                        }
                    },
                    {
                        "text": "500",
                        "estimates": {
                            "voucherc": 0.028750747442245483,
                            "open_ai": 0.4
                        }
                    }
                ]
            },
            {
                "text": "Copyright 2021, New Balance",
                "estimates": {
                    "dealc": 0.2749166666666667
                },
                "words": [
                    {
                        "text": "Copyright",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.005517333745956421
                        }
                    },
                    {
                        "text": "2021",
                        "estimates": {
                            "voucherc": 0.1932232677936554,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "New",
                        "estimates": {
                            "voucherc": 0.0006843507289886475,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "Balance",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.010602414608001707
                        }
                    }
                ]
            },
            {
                "text": "Privacy Policy / Returns Policy / Terms and Conditions",
                "estimates": {
                    "dealc": 0.000029999999999999997
                },
                "words": [
                    {
                        "text": "Privacy",
                        "estimates": {
                            "voucherc": 0.010602414608001707,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "Policy",
                        "estimates": {
                            "voucherc": 0.014663994312286375,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "Returns",
                        "estimates": {
                            "voucherc": 0.010602414608001707,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "Policy",
                        "estimates": {
                            "voucherc": 0.014664024114608765,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "Terms",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.017828315496444702
                        }
                    },
                    {
                        "text": "and",
                        "estimates": {
                            "voucherc": 0.0020304322242736816,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "Conditions",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.004759609699249268
                        }
                    }
                ]
            },
            {
                "text": "New Balance Customer Care",
                "estimates": {
                    "dealc": 0.13349648926320634
                },
                "words": [
                    {
                        "text": "New",
                        "estimates": {
                            "voucherc": 0.0006843507289886475,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "Balance",
                        "estimates": {
                            "voucherc": 0.010602414608001707,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "Customer",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.007294446229934692
                        }
                    },
                    {
                        "text": "Care",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.01020154356956482
                        }
                    }
                ]
            },
            {
                "text": "New Balance Athletic Shoes (UK) Ltd, Appleton House",
                "estimates": {
                    "dealc": 0.27611922443699666
                },
                "words": [
                    {
                        "text": "New",
                        "estimates": {
                            "voucherc": 0.0006843507289886475,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "Balance",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.010602414608001707
                        }
                    },
                    {
                        "text": "Athletic",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.007294446229934692
                        }
                    },
                    {
                        "text": "Shoes",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.017828315496444702
                        }
                    },
                    {
                        "text": "Ltd",
                        "estimates": {
                            "voucherc": 0.0006843507289886475,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "Appleton",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.007294446229934692
                        }
                    },
                    {
                        "text": "House",
                        "estimates": {
                            "voucherc": 0.017828315496444702,
                            "open_ai": 0.4
                        }
                    }
                ]
            },
            {
                "text": "430 Birchwood Blvd, Warrington WA3 7WD",
                "estimates": {
                    "dealc": 0.036405536595762664
                },
                "words": [
                    {
                        "text": "430",
                        "estimates": {
                            "voucherc": 0.028750747442245483,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "Birchwood",
                        "estimates": {
                            "voucherc": 0.005517333745956421,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "Blvd",
                        "estimates": {
                            "voucherc": 0.01020154356956482,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "Warrington",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.004759609699249268
                        }
                    },
                    {
                        "text": "WA3",
                        "estimates": {
                            "voucherc": 0.770621657371521,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "7WD",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.036160409450531006
                        }
                    }
                ]
            },
            {
                "text": "If you want to unsubscribe from our mailing list, click here",
                "estimates": {
                    "dealc": 0.002
                },
                "words": [
                    {
                        "text": "you",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.0020304322242736816
                        }
                    },
                    {
                        "text": "want",
                        "estimates": {
                            "voucherc": 0.02872207760810852,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "unsubscribe",
                        "estimates": {
                            "voucherc": 0.003333181142807007,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "from",
                        "estimates": {
                            "voucherc": 0.02872207760810852,
                            "open_ai": 0.4
                        }
                    },
                    {
                        "text": "our",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.0020304322242736816
                        }
                    },
                    {
                        "text": "mailing",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.01134440302848816
                        }
                    },
                    {
                        "text": "list",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.02872207760810852
                        }
                    },
                    {
                        "text": "click",
                        "estimates": {
                            "open_ai": 0.4,
                            "voucherc": 0.019675999879837036
                        }
                    },
                    {
                        "text": "here",
                        "estimates": {
                            "voucherc": 0.02872207760810852,
                            "open_ai": 0.4
                        }
                    }
                ]
            }
        ]);

        serde_json::from_value(json).unwrap()
    }
}
