mod features;
mod types;

use clap::{App, Arg};
use rand::{seq::SliceRandom, thread_rng};
use smartcore::linalg::naive::dense_matrix::{BaseVector, DenseMatrix};
use smartcore::svm::{
    svr::{SVRParameters, SVR},
    Kernels,
};
use std::{
    fs,
    io::{self, BufRead},
    path::{Path, PathBuf},
    time::Instant,
};
use types::{Feature, SVM};

const DEFAULT_VOUCHERS_PATH: &str = "data/vouchers.txt";
const DEFAULT_NVOUCHERS_PATH: &str = "data/nvouchers.txt";

const TRAINING_SAMPLE_SIZE: usize = 8000;
const SVM_C: f64 = 2.0;
const SVM_E: f64 = 0.0005;
const RBF_GAMMA: f64 = 0.001;

fn main() {
    let mut app = App::new("voucherc_train_cli")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("vouchers")
                .long("vouchers")
                .help("Text file with one voucher per line")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("nvouchers")
                .long("nvouchers")
                .help("Text file with one word that's not a voucher per line")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("csv")
                .long("csv")
                .help("Use this flag to store CSV with features")
                .takes_value(false),
        );

    app.print_help().expect("Cannot print help");
    println!();
    println!();

    let matches = app.get_matches();

    let store_csv = matches.is_present("csv");

    let vouchers_path = matches
        .value_of("vouchers")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_VOUCHERS_PATH));
    println!("Vouchers file at {:?}", vouchers_path);

    let nvouchers_path = matches
        .value_of("nvouchers")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_NVOUCHERS_PATH));
    println!("Not vouchers file at {:?}", nvouchers_path);

    // read vouchers
    let (vt, vv) =
        read_features_into_train_and_test_sets(vouchers_path, store_csv);
    // read "not vouchers"
    let (nvt, nvv) =
        read_features_into_train_and_test_sets(nvouchers_path, store_csv);

    // put together train data from vouchers and "not vouchers"
    let training_data = {
        let d: Vec<&[_]> =
            vt.iter().chain(nvt.iter()).map(|v| v.as_slice()).collect();
        DenseMatrix::from_2d_array(&d)
    };
    let training_labels: Vec<_> = (0..vt.len())
        .map(|_| 1.0)
        .chain((0..nvt.len()).map(|_| 0.0))
        .collect();

    // run classification with mostly default parameters
    let since_classification_started = Instant::now();
    println!();
    println!("Running classification");
    let svm: SVM = SVR::fit(
        &training_data,
        &training_labels,
        SVRParameters::default()
            .with_c(SVM_C)
            .with_eps(SVM_E)
            .with_kernel(Kernels::rbf(RBF_GAMMA)),
    )
    .unwrap();
    println!(
        "Classification done in {}ms",
        since_classification_started.elapsed().as_millis()
    );

    // evaluate the results
    let gv: Vec<_> = svm
        .predict(&DenseMatrix::from_2d_vec(&vv))
        .unwrap()
        .into_iter()
        .map(|f| f.clamp(0.0, 1.0))
        .collect();
    assert_eq!(gv.len(), vv.len());
    let ve = (vv.len() as f64 - gv.sum()) / vv.len() as f64;
    println!("vouchers error     : {:.3}", ve);

    let gnv: Vec<_> = svm
        .predict(&DenseMatrix::from_2d_vec(&nvv))
        .unwrap()
        .into_iter()
        .map(|f| f.clamp(0.0, 1.0))
        .collect();
    let gve = gnv.sum() / nvv.len() as f64;
    println!("not-vouchers error : {:.3}", gve);

    println!();
    println!("average error      : {:.3}", (ve + gve) / 2.0);

    println!();
    println!("Persist model to data/svm.json? [Yn]");
    let stdin = io::stdin();
    let user_input = stdin.lock().lines().next().unwrap().unwrap();

    if user_input.to_lowercase() == "y" {
        // and finally persist the result (in version control too) so that the http
        // bin can read it
        let json = serde_json::to_string(&svm).unwrap();
        fs::write("data/svm.json", json).unwrap();
        println!("Done");
    } else {
        println!("Nothing to do");
    }
}

fn read_features_into_train_and_test_sets(
    path: impl AsRef<Path>,
    store_csv: bool,
) -> (Vec<Feature>, Vec<Feature>) {
    let mut rng = thread_rng();
    let path = path.as_ref();

    println!("Reading file {:?}", path);
    let input = fs::read_to_string(&path).expect("Cannot read input file");
    let lines = input.lines();

    let (lower_bound, upper_bound) = lines.size_hint();
    let mut features: Vec<Feature> =
        Vec::with_capacity(upper_bound.unwrap_or(lower_bound));

    for word in lines {
        features.push(features::from_word(&word));
    }

    features.shuffle(&mut rng);

    if store_csv {
        let csv_path = path.with_extension("csv");
        let mut csv =
            csv::Writer::from_path(csv_path).expect("Cannot write CSV");
        features.iter().for_each(|f| csv.serialize(f).unwrap());
        csv.flush().expect("Cannot flush CSV");
    }

    // reads data set and splits it in two parts, train and test sets
    assert!(
        TRAINING_SAMPLE_SIZE < features.len(),
        "Training sample size too high"
    );
    let train: Vec<_> = features.drain(0..TRAINING_SAMPLE_SIZE).collect();
    let test = features;
    (train, test)
}
