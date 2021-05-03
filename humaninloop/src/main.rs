mod html;
mod models;
mod prelude;
mod state;

use crate::models::WordWithEstimate;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use serde::Deserialize;
use shared::{vision::Annotation, S3Ext};
use state::State;
use tokio::fs;

#[actix_web::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    log::info!("Starting humaninloop v{}", env!("CARGO_PKG_VERSION"));

    let state = State::new().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/images/{id}/classification")
                    .route(web::get().to(show_image))
                    .route(web::post().to(evaluate_image)),
            )
    })
    .bind("127.0.0.1:8888")
    .expect("Cannot start web server")
    .run()
    .await
    .expect("Web server died");
}

#[derive(Deserialize)]
struct EvaluateImageForm {
    json: String,
}

async fn evaluate_image(
    state: web::Data<State>,
    newsletter_id: web::Path<String>,
    body: web::Form<EvaluateImageForm>,
) -> HttpResponse {
    log::debug!("Received JSON for newsletter {}", newsletter_id);

    fs::write(format!(".local/{}.json", newsletter_id), &body.json)
        .await
        .expect("Cannot write json file");

    let redirect_url = state
        .newsletter_ids
        .lock()
        .unwrap()
        .pop()
        .map(|id| format!("/images/{}/classification", id))
        .unwrap_or_else(|| "https://porkbrain.com".to_string());

    let updated_ids = state
        .newsletter_ids
        .lock()
        .unwrap()
        .iter()
        .cloned()
        .collect::<Vec<_>>()
        .join("\n");
    fs::write("humaninloop/.local/keys.txt", updated_ids)
        .await
        .unwrap();

    HttpResponse::SeeOther()
        .append_header(("Location", redirect_url.as_str()))
        .finish()
}

async fn show_image(
    state: web::Data<State>,
    newsletter_id: web::Path<String>,
) -> HttpResponse {
    let annotation: Annotation = state
        .s3
        .get("newsletter-ocr-na5d".to_string(), newsletter_id.to_string())
        .await
        .expect("Cannot read OCR object")
        .map(|bytes| serde_json::from_slice(&bytes).ok())
        .flatten()
        .expect("OCR object has empty body");

    let pwords = annotation.words;
    log::trace!("Annotating {} words in {}", pwords.len(), newsletter_id);
    let twords: Vec<_> = pwords.iter().map(|w| &w.word).collect();
    let estimates: Vec<f64> = surf::post("http://127.0.0.1:8080")
        .body(serde_json::to_value(&twords).unwrap())
        .await
        .unwrap()
        .take_body()
        .into_json()
        .await
        .unwrap();
    assert_eq!(estimates.len(), pwords.len());
    let mut words: Vec<_> = pwords
        .into_iter()
        .zip(estimates)
        .map(|(word, estimate)| WordWithEstimate {
            word,
            estimate,
            is_in_phrase: false,
        })
        .collect();

    let lines: Vec<_> = annotation.text.lines().collect();
    log::trace!("Annotating {} phrases in {}", lines.len(), newsletter_id);
    let estimates: Vec<f64> = surf::post("http://127.0.0.1:8081")
        .body(serde_json::to_value(&lines).unwrap())
        .await
        .unwrap()
        .take_body()
        .into_json()
        .await
        .unwrap();
    assert_eq!(estimates.len(), lines.len());
    let mut line_index = 0;
    for word in &mut words {
        if !lines[line_index].contains(word.word.word.as_str()) {
            line_index += 1;
        }

        if estimates[line_index] > 0.7 {
            word.is_in_phrase = true;
        }
    }

    let url = format!(
        "https://newsletter-screenshot-4fj0.s3-eu-west-1.amazonaws.com/{}",
        newsletter_id
    );

    let html = state
        .template
        .render_image_page(&newsletter_id, &url, &words)
        .expect("Cannot render image page");
    HttpResponse::Ok()
        .append_header(("Content-Type", "text/html"))
        .body(html)
}
