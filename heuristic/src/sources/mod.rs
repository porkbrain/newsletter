mod common_phrases;
mod openai;

use crate::models::{Phrases, Source};
use crate::prelude::*;
use shared::http;

pub async fn get_phrases_with_estimates(
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
        common_phrases::word_estimates(&document.words_str());
    if let Some(estimates) = common_phrases_estimates {
        document.apply_words_estimates(Source::CommonPhrases, estimates)?;
    }

    // send some promising phrases to openai to check them out
    let openai_estimates =
        openai::word_estimates(conf, http_client, document.inner()).await?;
    if let Some(estimates) = openai_estimates {
        document.apply_words_estimates(Source::OpenAi, estimates)?;
    }

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
    document.apply_phrases_estimates(Source::Dealc, dealc_estimates)?;

    // fetch estimates for how likely each word is a voucher
    let voucherc_estimates: Vec<f64> = {
        let words_json = serde_json::to_value(document.words_str())?;
        let voucherc_res_body = http_client
            .post_json(&conf.voucherc_url, &words_json)
            .await?;

        serde_json::from_slice(&voucherc_res_body)?
    };
    document.apply_words_estimates(Source::Voucherc, voucherc_estimates)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
