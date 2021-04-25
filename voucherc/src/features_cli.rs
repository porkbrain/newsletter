//! Reads each line in the given file, and creates related output line which
//! marks what features does the input have. To be used with a text file of
//! voucher codes, and then repeated with a text file of non voucher codes.
//! The produced outputs can be used to classify words based on their features.
//!
//! # Options
//! * -i, --input File with one word per line
//! * -o, --output File csv output will be stored

mod features;
mod types;

use clap::{App, Arg};
use std::{fs, path::PathBuf};

fn main() {
    let mut app = App::new("voucherc_features_cli")
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
    let mut features: Vec<String> =
        Vec::with_capacity(upper_bound.unwrap_or(lower_bound));

    for word in lines {
        features.push(
            features::from_word(&word)
                .into_iter()
                .map(|f| format!("{:.1}", f))
                .collect(),
        );
    }

    fs::write(output_path, features.join("\n")).expect("Cannot write output");

    println!("Done");
}
