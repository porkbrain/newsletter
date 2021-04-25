mod conf;
mod features;
mod types;

use conf::Conf;
use dotenv::dotenv;
use smartcore::linalg::naive::dense_matrix::DenseMatrix;
use std::sync::Arc;
use tide::{Request, Response};
use types::SVM;

const SVM_JSON: &str = include_str!("../data/svm.json");

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();
    env_logger::init();
    log::info!("Starting voucherc v{}", env!("CARGO_PKG_VERSION"));

    let conf = envy::from_env::<Conf>().expect("Cannot build config");

    let mut app = tide::with_state(State::new());
    app.at("/words").post(classify_words);
    app.listen(conf.http_address).await?;

    Ok(())
}

async fn classify_words(mut req: Request<State>) -> tide::Result {
    let words: Vec<String> = req.body_json().await?;
    let features: Vec<_> =
        words.iter().map(|w| features::from_word(&w)).collect();

    let inference = req
        .state()
        .svm
        .predict(&DenseMatrix::from_2d_vec(&features));

    match inference {
        Ok(classes) => match serde_json::to_value(&classes) {
            Ok(json) => Ok(Response::builder(200).body(json).build()),
            Err(e) => {
                log::error!(
                    "Cannot json stringify {} classes due to {}",
                    classes.len(),
                    e
                );
                Ok(Response::builder(500).build())
            }
        },
        Err(e) => {
            log::error!(
                "Cannot classify {} features due to {}",
                features.len(),
                e
            );
            Ok(Response::builder(500).build())
        }
    }
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

