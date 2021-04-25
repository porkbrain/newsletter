use crate::types::Feature;
use static_init::dynamic;
use std::collections::HashSet;

#[dynamic]
static DICT: HashSet<String> = include_str!("../data/dictionary.en.txt")
    .lines()
    .map(ToOwned::to_owned)
    .collect();

#[allow(dead_code)]
pub fn from_word(w: &str) -> Feature {
    let b2f = |b| if b { 1.0 } else { 0.0 };

    vec![
        w.len() as f64,
        b2f(is_lowercase(&w)),
        b2f(is_uppercase(&w)),
        b2f(has_letters(&w)),
        b2f(has_letters_only(&w)),
        b2f(has_digits(&w)),
        b2f(has_digits_only(&w)),
        b2f(are_more_than_half_digits(&w)),
        b2f(is_alphanumeric_or_dash_or_underscore(&w)),
        b2f(has_letters_which_end_with_two_digits(&w)),
        b2f(ends_with_digits(&w)),
        b2f(is_in_english_dictionary(&w)),
    ]
}

fn is_lowercase(w: &str) -> bool {
    w.matches(char::is_uppercase).next().is_none()
        && w.matches(char::is_lowercase).next().is_some()
}

fn is_uppercase(w: &str) -> bool {
    w.matches(char::is_uppercase).next().is_some()
        && w.matches(char::is_lowercase).next().is_none()
}

fn has_letters(w: &str) -> bool {
    w.chars().any(char::is_alphabetic)
}

fn has_letters_only(w: &str) -> bool {
    !w.is_empty() && w.chars().all(char::is_alphabetic)
}

fn has_digits(w: &str) -> bool {
    w.chars().any(char::is_numeric)
}

fn has_digits_only(w: &str) -> bool {
    !w.is_empty() && w.chars().all(char::is_numeric)
}

fn are_more_than_half_digits(w: &str) -> bool {
    w.len() / 2 < w.chars().filter(|c| char::is_numeric(*c)).count()
}

fn is_alphanumeric_or_dash_or_underscore(w: &str) -> bool {
    !w.is_empty()
        && w.chars()
            .all(|c| char::is_alphanumeric(c) || c == '_' || c == '-')
}

fn ends_with_digits(w: &str) -> bool {
    w.chars().last().filter(|c| char::is_numeric(*c)).is_some()
}

fn has_letters_which_end_with_two_digits(w: &str) -> bool {
    let mut chars = w.chars().rev().peekable();
    chars.next().filter(|c| char::is_numeric(*c)).is_some()
        && chars.next().filter(|c| char::is_numeric(*c)).is_some()
        && chars.peek().is_some()
        && chars.all(char::is_alphabetic)
}

fn is_in_english_dictionary(w: &str) -> bool {
    DICT.contains(&w.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_lowercase() {
        assert!(is_lowercase("this123-_"));
        assert!(!is_lowercase("thIs123-_"));
        assert!(!is_lowercase("123-_"));
        assert!(!is_lowercase("AHOJ123-_"));
        assert!(!is_lowercase(""));
    }

    #[test]
    fn test_is_uppercase() {
        assert!(is_uppercase("THIS123-_"));
        assert!(!is_uppercase("thIs123-_"));
        assert!(!is_uppercase("123-_"));
        assert!(!is_uppercase("ahoj123-_"));
        assert!(!is_uppercase(""));
    }

    #[test]
    fn test_has_letters_only() {
        assert!(has_letters_only("onlyleTTErs"));
        assert!(!has_letters_only("123TErs"));
        assert!(!has_letters_only(""));
        assert!(!has_letters_only("-_"));
        assert!(!has_letters_only("123"));
    }

    #[test]
    fn test_has_letters() {
        assert!(has_letters("onlyleTTErs"));
        assert!(has_letters("123TErs"));
        assert!(!has_letters(""));
        assert!(!has_letters("-_"));
        assert!(!has_letters("123"));
    }

    #[test]
    fn test_has_digits_only() {
        assert!(has_digits_only("123"));
        assert!(!has_digits_only("onlyleTTErs"));
        assert!(!has_digits_only("123TErs"));
        assert!(!has_digits_only(""));
        assert!(!has_digits_only("-_"));
    }

    #[test]
    fn test_has_digits() {
        assert!(has_digits("123"));
        assert!(has_digits("123TErs"));
        assert!(!has_digits("onlyleTTErs"));
        assert!(!has_digits(""));
        assert!(!has_digits("-_"));
    }

    #[test]
    fn test_are_more_than_half_digits() {
        assert!(are_more_than_half_digits("123"));
        assert!(are_more_than_half_digits("1234abc"));
        assert!(!are_more_than_half_digits("at_123"));
        assert!(!are_more_than_half_digits("onlyleTTErs"));
        assert!(!are_more_than_half_digits(""));
        assert!(!are_more_than_half_digits("1a"));
    }

    #[test]
    fn test_is_alphanumeric_or_dash_or_underscore() {
        assert!(is_alphanumeric_or_dash_or_underscore("123"));
        assert!(is_alphanumeric_or_dash_or_underscore("1234abc"));
        assert!(is_alphanumeric_or_dash_or_underscore("at_123"));
        assert!(is_alphanumeric_or_dash_or_underscore("-123-_onlyleTTErs"));
        assert!(!is_alphanumeric_or_dash_or_underscore(""));
        assert!(!is_alphanumeric_or_dash_or_underscore("1a$"));
    }

    #[test]
    fn test_ends_with_digits() {
        assert!(ends_with_digits("123"));
        assert!(ends_with_digits("at_123"));
        assert!(!ends_with_digits("1234abc"));
        assert!(!ends_with_digits("-123-_onlyleTTErs"));
        assert!(!ends_with_digits(""));
        assert!(!ends_with_digits("1a$"));
    }

    #[test]
    fn test_has_letters_which_end_with_two_digits() {
        assert!(has_letters_which_end_with_two_digits("OFF20"));
        assert!(!has_letters_which_end_with_two_digits("123hehe12"));
        assert!(!has_letters_which_end_with_two_digits("23"));
        assert!(!has_letters_which_end_with_two_digits("123"));
        assert!(!has_letters_which_end_with_two_digits("123hehe1"));
    }

    #[test]
    fn test_is_in_english_dictionary() {
        assert!(is_in_english_dictionary("dog"));
        assert!(is_in_english_dictionary("DOG"));
        assert!(!is_in_english_dictionary("d4wg"));
        assert!(!is_in_english_dictionary("nepravdepodobne"));
    }
}
