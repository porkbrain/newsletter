mod conf;
mod features;
mod types;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use conf::Conf;
use dotenv::dotenv;
use smartcore::linalg::naive::dense_matrix::DenseMatrix;
use std::{process::Stdio, sync::Arc};
use tokio::process::Command;
use types::SVM;
use tokio::io::AsyncWriteExt;

const SVM_JSON: &str = include_str!("../data/svm.json");

#[actix_web::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    log::info!("Starting voucherc v{}", env!("CARGO_PKG_VERSION"));

    let conf = envy::from_env::<Conf>().expect("Cannot build config");

    HttpServer::new(move || {
        App::new()
            .app_data(State::new())
            .wrap(middleware::Logger::default())
            .service(web::resource("/words").to(classify_words))
    })
    .bind(conf.http_address)
    .expect("Cannot start web server")
    .run()
    .await
    .expect("Web server died");
}

async fn classify_words(
    state: web::Data<State>,
    words: web::Json<Vec<String>>,
) -> HttpResponse {
    log::debug!("Classifying {} words", words.len());

    let features: Vec<_> =
        words.iter().map(|w| features::from_word(w)).collect();

    if features.is_empty() {
        return HttpResponse::Ok().finish();
    }

    // spawn new process which boots the neural net
    let mut dnn = Command::new("python3")
        .arg("py-nn/src/predict.py")
        .arg("data/dnn_model")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    // write the features to stdin so that the process can predict their classes
    let mut csv = csv::Writer::from_writer(vec![]);
    features.iter().for_each(|f| csv.serialize(f).unwrap());
    let features_csv = String::from_utf8(csv.into_inner().expect(
        "Cannot write
            csv data",
    ))
    .expect("Cannot create string from csv");
    dnn.stdin
        .as_mut()
        .expect("Failed to open stdin")
        .write_all(features_csv.as_bytes())
        .await
        .expect("Failed to write to stdin");

    // while dnn process is working, calculate svm inferences
    let svm_inference = state
        .svm
        .predict(&DenseMatrix::from_2d_vec(&features))
        .expect("Cannot predict features with svm");

    // convert the output to vector of floats
    let dnn_stdout = dnn.wait_with_output().await.expect("Failed to read stdout").stdout;
    let dnn_inferences: Vec<_> = String::from_utf8(dnn_stdout)
        .expect("Cannot parse python output")
        .lines()
        .filter_map(|n| n.parse::<f64>().ok())
        .collect();

    assert_eq!(dnn_inferences.len(), svm_inference.len());

    let inferences: Vec<_> = dnn_inferences
        .into_iter()
        .zip(svm_inference.into_iter())
        .map(|(a, b)| (a + b) / 2.0)
        .collect();

    HttpResponse::Ok().json(&inferences)
}

#[derive(Clone)]
struct State {
    svm: Arc<SVM>,
}

impl State {
    fn new() -> Self {
        let svm: SVM =
            serde_json::from_str(SVM_JSON).expect("Cannot build SVM state");

        Self { svm: Arc::new(svm) }
    }
}
