mod common_phrases;
//mod openai;

use crate::prelude::*;
use shared::http;
use shared::phrases::{Phrases, Source};

pub async fn deals_and_vouchers(
    conf: &Conf,
    http_client: &dyn http::Client,
    text: &str,
) -> Result<Phrases, Error> {
    let mut document = Phrases::from_text(text);

    apply_dealc_and_voucherc_estimates(conf, http_client, &mut document)
        .await?;

    // if there are some some common newsletter phrases (USE CODE ABC20), then
    // apply estimates from those
    let common_phrases_estimates =
        common_phrases::word_estimates(document.words().as_slice());
    if let Some(estimates) = common_phrases_estimates {
        apply_words_estimates(&mut document, Source::CommonPhrases, estimates)?;
    }

    // send some promising phrases to openai to check them out
    /*
    let openai_estimates =
        openai::word_estimates(conf, http_client, document.inner()).await?;
    if let Some(estimates) = openai_estimates {
        document.apply_words_estimates(Source::OpenAi, estimates)?;
    }
    */

    Ok(document)
}

async fn apply_dealc_and_voucherc_estimates(
    conf: &Conf,
    http_client: &dyn http::Client,
    document: &mut Phrases,
) -> Result<(), Error> {
    // fetch estimates for how likely each phrase is a deal
    // TODO: can be made concurrent with next step
    let dealc_estimates: Vec<f64> = {
        let phrases_json = serde_json::to_value(&document.phrases_str())?;
        let dealc_res_body = http_client
            .post_json(&conf.dealc_url, &phrases_json)
            .await?;

        serde_json::from_slice(&dealc_res_body)?
    };
    apply_phrases_estimates(document, Source::Dealc, dealc_estimates)?;

    // fetch estimates for how likely each word is a voucher
    let voucherc_estimates: Vec<f64> = {
        let words_json = serde_json::to_value(document.words_str())?;
        let voucherc_res_body = http_client
            .post_json(&conf.voucherc_url, &words_json)
            .await?;

        serde_json::from_slice(&voucherc_res_body)?
    };
    apply_words_estimates(document, Source::Voucherc, voucherc_estimates)?;

    Ok(())
}

pub fn apply_phrases_estimates(
    document: &mut Phrases,
    source: Source,
    estimates: Vec<f64>,
) -> Result<(), Error> {
    let phrases = document.inner_mut();
    if phrases.len() != estimates.len() {
        return Err(Error::new(format!(
            "Got {} phrases, but {} estimates",
            phrases.len(),
            estimates.len()
        )));
    }

    for (phrase, estimate) in phrases.into_iter().zip(estimates.into_iter()) {
        phrase.estimates.insert(source, estimate);
    }

    Ok(())
}

pub fn apply_words_estimates(
    document: &mut Phrases,
    source: Source,
    estimates: Vec<f64>,
) -> Result<(), Error> {
    let words = document.words_mut();
    if words.len() != estimates.len() {
        return Err(Error::new(format!(
            "Got {} words, but {} estimates",
            words.len(),
            estimates.len()
        )));
    }

    for (w, estimate) in words.into_iter().zip(estimates.into_iter()) {
        w.estimates.insert(source, estimate);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use shared::reqwest;

    #[ignore]
    #[tokio::test]
    async fn it_applies_dealc_and_voucherc_estimates() {
        let conf = Conf {
            dealc_url: "http://localhost:8081".to_string(),
            voucherc_url: "http://localhost:8080".to_string(),
            ..Default::default()
        };
        let http_client = reqwest::Client::new();

        let text = "🛒 26% Off HUGE Savings!\nShop ‘Til You Drop |\n\
                    View online\nBLACK FRIDAY OFFERS\nSHORT BREAKS\nGOURMET\n\
                    DRIVING\nSPA DAYS\nDAYS OUT\nCHRISTMAS\n\
                    SHOP ’TIL YOU DROP\nUse code\nBLACK26\nat the checkout\n\
                    SHOP NOW\nSHOP BLACK FRIDAY DEALS\nDeals Under £50\n\
                    Up To £100 Off Best Sellers\nUp To 50% Off Christmas\n\
                    Christmas Gifts For Couples\nChristmas Gifts For Him\n\
                    Christmas Gifts For Her\nChristmas Gifts For Children\n\
                    Our COVID Guarantee\nBuy with added peace of mind. \
                    If you are unable to book and take your experience due \
                    to COVID we will extend or exchange your voucher \
                    free of charge depending on your preference – we are all \
                    in this together.\nLET'S BE FRIENDS\nWe are proud to be \
                    only carbon neutral gift experience company in the UK. \
                    Click\nhere\nto learn more!\nUpdate Preferences\n\
                    Unsubscribe\nRefer A Friend\nContact Us\n\
                    This email was sent to you\nnewsletter@porkbrain.com\n\
                    by Buyagift because\nyou gave us your email address along \
                    with permission to contact you.\n*Discount code is valid \
                    until midnight, Sunday 29th November 2020. To use simply \
                    enter the code\nBLACK26\nin the basket and click 'go'. \
                    Discount codes are issued subject to availability and can \
                    be withdrawn without notice at any time. Only one discount \
                    may be used per order and these cannot be used against \
                    exchanges, extensions, delivery, gift packs or any other \
                    facility provided by Buyagift. Discounts can only be \
                    applied to internet orders on\nwww.buyagift.co.uk\n\
                    Discount codes cannot be used when exchanging experience \
                    vouchers or when redeeming credit\nvouchers and money \
                    vouchers on\nwww.buyagift.com/myvoucher\n\
                    Buyagift reserves the right to stop discount codes being \
                    used against specific products. For full terms and \
                    conditions, please visit\nterms\n*You may find that some \
                    of our amazing special offers are valid for a little less \
                    time – please see individual products for more details\n\
                    To view our Privacy Policy, click\nhere\n\
                    Buyagift, 4 Imperial Place, Maxwell Road, Borehamwood, \
                    Hertfordshire, WD6 1JN\n- - - - - - - - - - - - - -";

        let mut phrases = Phrases::from_text(text);

        apply_dealc_and_voucherc_estimates(&conf, &http_client, &mut phrases)
            .await
            .expect("Cannot get phrases with estimates");

        panic!("{:#?}", phrases);
    }

    #[test]
    fn it_serializes() {
        let mut phrases =
            Phrases::from_text("first phrase and then\nthere is second phrase");

        apply_phrases_estimates(&mut phrases, Source::Dealc, vec![0.8, 0.7])
            .unwrap();
        apply_words_estimates(
            &mut phrases,
            Source::OpenAi,
            (0..7).map(|_| 0.5).collect(),
        )
        .unwrap();

        assert_eq!(
            json!([
                    {
                        "text": "first phrase and then",
                        "estimates": { "dealc": 0.8 },
                        "words": [
                            {
                                "text": "first",
                                "estimates": { "open_ai": 0.5 },
                            },
                            {
                                "text": "phrase",
                                "estimates": { "open_ai": 0.5 },
                            },
                            {
                                "text": "and",
                                "estimates": { "open_ai": 0.5 },
                            },
                            {
                                "text": "then",
                                "estimates": { "open_ai": 0.5 },
                            },
                        ]
                    },
                    {
                        "text": "there is second phrase",
                        "estimates": { "dealc": 0.7 },
                        "words": [
                            {
                                "text": "there",
                                "estimates": { "open_ai": 0.5 },
                            },
                            {
                                "text": "second",
                                "estimates": { "open_ai": 0.5 },
                            },
                            {
                                "text": "phrase",
                                "estimates": { "open_ai": 0.5 },
                            },
                        ]
                    }
            ]),
            serde_json::to_value(&phrases).unwrap()
        );
    }
}
