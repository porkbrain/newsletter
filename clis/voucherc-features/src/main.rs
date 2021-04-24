//! Reads each line in the given file, and creates related output line which
//! marks what features does the input have. To be used with a text file of
//! voucher codes, and then repeated with a text file of non voucher codes.
//! The produced outputs can be used to classify words based on their features.
//!
//! # Options
//! * -i, --input File with one word per line
//! * -o, --output File csv output will be stored

use clap::{App, Arg};
use static_init::dynamic;
use std::{collections::HashSet, fs, path::PathBuf};

#[dynamic]
static DICT: HashSet<String> = include_str!("dict.txt")
    .lines()
    .map(ToOwned::to_owned)
    .collect();

fn main() {
    let mut app = App::new("voucherc-features")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .help("File with one word per line")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("File csv output will be stored")
                .takes_value(true),
        );

    app.print_help().expect("Cannot print help");
    println!();

    let matches = app.get_matches();

    let input_path = matches
        .value_of("input")
        .map(PathBuf::from)
        .expect("Provide path to the input file with -i");
    println!("Processing file {:?}", input_path);

    let output_path = matches
        .value_of("output")
        .map(PathBuf::from)
        .expect("Provide path where the csv output should be stored with -o");
    println!("Writing output to {:?}", output_path);

    let input = fs::read_to_string(input_path).expect("Cannot read input file");
    let lines = input.lines();

    let (lower_bound, upper_bound) = lines.size_hint();
    let mut features = Vec::with_capacity(upper_bound.unwrap_or(lower_bound));

    for word in lines {
        let b2b = |b| if b { "1.0" } else { "0.0" };
        features.push(format!(
            "{}.0,{},{},{},{},{},{},{},{},{},{},{}",
            word.len(),
            b2b(is_lowercase(&word)),
            b2b(is_uppercase(&word)),
            b2b(has_letters(&word)),
            b2b(has_letters_only(&word)),
            b2b(has_digits(&word)),
            b2b(has_digits_only(&word)),
            b2b(are_more_than_half_digits(&word)),
            b2b(is_alphanumeric_or_dash_or_underscore(&word)),
            b2b(has_letters_which_end_with_two_digits(&word)),
            b2b(ends_with_digits(&word)),
            b2b(is_in_english_dictionary(&word))
        ));
    }

    fs::write(output_path, features.join("\n")).expect("Cannot write output");

    println!("Done");
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
