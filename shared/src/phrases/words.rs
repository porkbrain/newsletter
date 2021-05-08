/// Returns list of sanitized words in first term, and their raw form in second.
pub fn from_phrase(phrase: &str) -> Vec<(String, String)> {
    const VOUCHER_MAX_LEN: usize = 32;
    const VOUCHER_MIN_LEN: usize = 3;

    const TRIM_CHARS_FROM_WORD: &[char] =
        &['\t', '"', '\'', ',', '.', '?', '!', ')', '(', ':', '*'];

    phrase
        .replace('\n', " ")
        .split(' ')
        .map(|s| (s.trim().trim_matches(TRIM_CHARS_FROM_WORD), s))
        .filter(|(s, _)| !s.is_empty())
        .filter(|(s, _)| (VOUCHER_MIN_LEN..=VOUCHER_MAX_LEN).contains(&s.len()))
        .map(|(sanitized, raw)| (sanitized.to_string(), raw.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_openai_output() {
        assert_correct_words_output(
            from_phrase(
                "SUMMER20.\n\
            The",
            ),
            vec!["SUMMER20", "The"],
            vec!["SUMMER20.", "The"],
        );
    }

    #[test]
    fn it_sanitizes_words() {
        assert_correct_words_output(
            from_phrase("Hello. "),
            vec!["Hello"],
            vec!["Hello."],
        );
        assert_correct_words_output(
            from_phrase("there!"),
            vec!["there"],
            vec!["there!"],
        );
        assert_correct_words_output(
            from_phrase("Code:"),
            vec!["Code"],
            vec!["Code:"],
        );
        assert_correct_words_output(
            from_phrase("~~nicky.~.~?"),
            vec!["~~nicky.~.~"],
            vec!["~~nicky.~.~?"],
        );
        assert_correct_words_output(
            from_phrase("I've"),
            vec!["I've"],
            vec!["I've"],
        );
        assert_correct_words_output(
            from_phrase("(been)"),
            vec!["been"],
            vec!["(been)"],
        );
        assert_correct_words_output(
            from_phrase("looking"),
            vec!["looking"],
            vec!["looking"],
        );
        assert_correct_words_output(
            from_phrase("'you"),
            vec!["you"],
            vec!["'you"],
        );
        assert_correct_words_output(from_phrase("'n''"), vec![], vec![]);
        assert_correct_words_output(
            from_phrase(
                "thisisextremelylongwordwhichwillneverbeavoucherinmillionyears",
            ),
            vec![],
            vec![],
        );
        assert_correct_words_output(
            from_phrase("star*"),
            vec!["star"],
            vec!["star*"],
        );
    }

    fn assert_correct_words_output(
        output: Vec<(String, String)>,
        expected_sanitized: Vec<&str>,
        expected_raw: Vec<&str>,
    ) {
        assert_eq!(
            output.iter().map(|(s, _)| s.as_str()).collect::<Vec<_>>(),
            expected_sanitized
        );
        assert_eq!(
            output.iter().map(|(_, r)| r.as_str()).collect::<Vec<_>>(),
            expected_raw
        );
    }
}
