pub fn from_text(text: &str) -> Vec<String> {
    let look_ahead = 3;

    let lines: Vec<_> = text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();

    let padding = (0..(3 + lines.len() % look_ahead)).map(|_| (false, ""));
    let lines: Vec<_> = lines
        .into_iter()
        .map(|l| {
            let is_independent = count_spaces(l) > 1;
            (is_independent, l)
        })
        .chain(padding)
        .collect();
    debug_assert_eq!(0, lines.len() % 3);

    let mut out_lines = vec![];

    let mut line_buffer = String::new();
    for window in lines.windows(look_ahead) {
        assert_eq!(look_ahead, 3);
        let (curr_indep, curr) = window[0];
        let (next1_indep, next1) = window[1];
        let (next2_indep, _) = window[2];

        if curr_indep && !next1_indep && next2_indep {
            out_lines.push(format!("{} {}", curr, next1));
        } else if curr_indep {
            // either next is independent, or both 2 dependent
            out_lines.push(curr.to_string());
        }
        // curr_indep is false
        else if next1_indep || count_spaces(&line_buffer) > 5 {
            if !line_buffer.is_empty() {
                line_buffer.push(' ');
                line_buffer.push_str(curr);
                out_lines.push(line_buffer.clone());
                line_buffer.clear();
            } else {
                // already appended to previous phrase
            }
        }
        // curr_indep and next1_indep are false
        else {
            if !curr.is_empty() {
                if !line_buffer.is_empty() {
                    line_buffer.push(' ')
                }
                line_buffer.push_str(curr);
            }
        }
    }

    if !line_buffer.is_empty() {
        out_lines.push(line_buffer);
    }

    out_lines
}

fn count_spaces(s: &str) -> usize {
    s.chars().filter(|c| *c == ' ').count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_email_to_list_of_phrases() {
        assert_text_eq_list(
            "View this email in your browser\n\
                WAREHOUSE\nNEW IN\nDRESSES\nTOPS\nACCESSORIES\n\
                TAKE 20% OFF EVERYTHING\nSHOP NOW\nWH ICONS\nVOL. 4\n\
                FEATURING\nCAROLINA\nElevate wardrobe foundations with\n\
                pinstripes, soft tailoring, denim\n\
                necessities and fresh florals.\nSHOP NEW IN\nDOUBLE\nTAKE\n\
                shop tailoring\nFLORAL\nARRANGEMENTS\nshop dresses\n\
                REFRESHED\nSTAPLES\nshop denim\nSHOP NOW\nGET\n\
                40% OFF THE SPRING REFRESH\nSHOP NOW\nDOWNLOAD THE APP:\n\
                Download on the\nGET IT ON\nApp Store\nGoogle Play\n\
                40% off the Outdoor Edit for a limited time only.\n\
                *20% off everything already applied.\nf\nPrivacy policy\n\
                Unsubscribe\nWarehouse Fashions Online Limited (No. 12579412),\
                a Company registered in England and Wales\n\
                based at 49-51 Dale Street, Manchester M1 2HF.\n",
            &[
                "View this email in your browser",
                "WAREHOUSE NEW IN DRESSES TOPS ACCESSORIES",
                "TAKE 20% OFF EVERYTHING",
                "SHOP NOW WH ICONS VOL. 4 FEATURING CAROLINA",
                "Elevate wardrobe foundations with",
                "pinstripes, soft tailoring, denim",
                "necessities and fresh florals.",
                "SHOP NEW IN",
                "DOUBLE TAKE shop tailoring FLORAL ARRANGEMENTS shop dresses REFRESHED",
                "STAPLES shop denim SHOP NOW GET",
                "40% OFF THE SPRING REFRESH SHOP NOW",
                "DOWNLOAD THE APP:",
                "Download on the",
                "GET IT ON",
                "App Store Google Play",
                "40% off the Outdoor Edit for a limited time only.",
                "*20% off everything already applied.",
                "f Privacy policy Unsubscribe",
                "Warehouse Fashions Online Limited (No. 12579412),a Company registered in England and Wales",
                "based at 49-51 Dale Street, Manchester M1 2HF."
            ],
        );
    }

    #[test]
    fn it_handles_empty_last_line() {
        assert_text_eq_list(
            "This is a test\nFor last line\n  ",
            &["This is a test", "For last line"],
        );

        assert_text_eq_list(
            "This is a test\nDEP\nOK\n  ",
            &["This is a test", "DEP OK"],
        );

        assert_text_eq_list(
            "This is a test\nWith a line",
            &["This is a test", "With a line"],
        );
    }

    #[test]
    fn it_pushes_rest_of_line_buffer() {
        assert_text_eq_list(
            "This is a test\nTRAILING\n",
            &["This is a test", "TRAILING"],
        );
    }

    fn assert_text_eq_list(text: &str, phrases: &[&str]) {
        assert_eq!(
            from_text(text),
            phrases.iter().map(ToString::to_string).collect::<Vec<_>>()
        );
    }
}
