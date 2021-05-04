use crate::select::{Deal, Voucher};
use geo::{intersects::Intersects, Coordinate, Rect};
use shared::{anchor::Anchor, vision::Annotation};

pub fn find_hrefs_for_resources(
    mut anchors: Vec<Anchor>,
    annotation: Annotation,
    deals: &mut [Deal],
    vouchers: &mut [Voucher],
) -> Option<String> {
    // sort by area so that large links over the whole email don't get
    // automatically picked for every deal/voucher
    anchors.sort_by_cached_key(|a| a.width * a.height);

    let rwords: Vec<_> = annotation
        .words
        .iter()
        .map(|w| {
            Rect::new(
                Coordinate {
                    x: w.top_left.x,
                    y: w.top_left.y,
                },
                Coordinate {
                    x: w.bottom_right.x,
                    y: w.bottom_right.y,
                },
            )
        })
        .collect();

    let ranchors: Vec<_> = anchors
        .iter()
        .map(|a| {
            Rect::new(
                Coordinate {
                    x: a.left,
                    y: a.top,
                },
                Coordinate {
                    x: a.left + a.width,
                    y: a.top + a.height,
                },
            )
        })
        .collect();

    for deal in deals {
        if let Some(first_wi) = annotation.text.find(&deal.text) {
            let href = map_phrase_on_email(
                &rwords,
                &annotation.text[..first_wi],
                &deal.text,
            )
            .find_map(|rw| {
                ranchors.iter().enumerate().find_map(|(ai, ra)| {
                    if ra.intersects(rw) {
                        Some(anchors[ai].href.clone())
                    } else {
                        None
                    }
                })
            });

            deal.link = href;
        }
    }

    for voucher in vouchers {
        if let Some(first_email_wi) = annotation.text.find(&voucher.phrase) {
            if let Some(first_phrase_wi) = voucher.phrase.find(&voucher.text) {
                let voucher_wi = annotation.text[..first_phrase_wi]
                    .chars()
                    .filter(|c| *c == ' ')
                    .count();

                let rvoucher = map_phrase_on_email(
                    &rwords,
                    &annotation.text[..first_email_wi],
                    &voucher.phrase,
                )
                .nth(voucher_wi);

                voucher.link = rvoucher.and_then(|rvoucher| {
                    ranchors.iter().enumerate().find_map(|(ai, ra)| {
                        if ra.intersects(rvoucher) {
                            Some(anchors[ai].href.clone())
                        } else {
                            None
                        }
                    })
                });
            }
        }
    }

    None
}

fn map_phrase_on_email<'a>(
    rwords: &'a [Rect<i32>],
    email_text: &str,
    phrase_text: &str,
) -> impl Iterator<Item = &'a Rect<i32>> {
    let wi = email_text.chars().filter(|c| *c == ' ').count();

    // plus one because last word is not followed by a space
    let wl = phrase_text.chars().filter(|c| *c == ' ').count() + 1;

    rwords.iter().skip(wi).take(wl)
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::vision::{self, Point};

    #[test]
    fn it_works_for_deals() {
        let anchors = vec![
            Anchor {
                href: "test2".to_string(),
                top: 5,
                left: 5,
                width: 100,
                height: 100,
            },
            Anchor {
                href: "test1".to_string(),
                top: 19,
                left: 19,
                width: 5,
                height: 3,
            },
            Anchor {
                href: "test3".to_string(),
                top: 40,
                left: 40,
                width: 30,
                height: 30,
            },
        ];

        let mut annotation = annotation_from_str(
            "This is a deal ok? It might be \
            a bit weird, but it's a deal nonetheless.",
        );
        // for first deal
        annotation.words[1].top_left = Point { x: 20, y: 20 };
        annotation.words[1].bottom_right = Point { x: 25, y: 25 };
        // for third deal
        annotation.words[12].top_left = Point { x: 50, y: 20 };
        annotation.words[12].bottom_right = Point { x: 67, y: 80 };

        let mut deals = vec![
            Deal::new("This is a deal".to_string(), 0.0),
            Deal::new("This is a weird deal".to_string(), 0.0),
            Deal::new("weird, but it's".to_string(), 0.0),
        ];

        find_hrefs_for_resources(anchors, annotation, &mut deals, &mut vec![]);

        assert_eq!(deals[0].link, Some("test1".to_string()));
        assert_eq!(deals[1].link, None);
        assert_eq!(deals[2].link, Some("test3".to_string()));
    }

    #[test]
    fn it_works_for_vouchers() {
        let anchors = vec![
            Anchor {
                href: "test2".to_string(),
                top: 5,
                left: 5,
                width: 100,
                height: 100,
            },
            Anchor {
                href: "test1".to_string(),
                top: 19,
                left: 19,
                width: 5,
                height: 3,
            },
            Anchor {
                href: "test3".to_string(),
                top: 40,
                left: 40,
                width: 30,
                height: 30,
            },
        ];

        let mut annotation = annotation_from_str(
            "Use cupon code: ALPHA It's very good trust me. This is weird",
        );
        // for first voucher
        annotation.words[3].top_left = Point { x: 20, y: 20 };
        annotation.words[3].bottom_right = Point { x: 25, y: 25 };
        // for third voucher
        annotation.words[8].top_left = Point { x: 50, y: 20 };
        annotation.words[8].bottom_right = Point { x: 67, y: 80 };

        let mut vouchers = vec![
            Voucher::new("code: ALPHA".to_string(), "ALPHA".to_string(), 0.0),
            Voucher::new("This is a deal".to_string(), "deal".to_string(), 0.0),
            Voucher::new("trust me. This".to_string(), "me".to_string(), 0.0),
        ];

        find_hrefs_for_resources(
            anchors,
            annotation,
            &mut vec![],
            &mut vouchers,
        );

        assert_eq!(vouchers[0].link, Some("test1".to_string()));
        assert_eq!(vouchers[1].link, None);
        assert_eq!(vouchers[2].link, Some("test3".to_string()));
    }

    fn annotation_from_str(s: &str) -> Annotation {
        let words = s
            .split(' ')
            .map(|w| vision::Word {
                word: w.to_string(),
                ..Default::default()
            })
            .collect();

        Annotation {
            text: s.to_string(),
            words,
        }
    }
}
