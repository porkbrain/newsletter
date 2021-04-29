mod html;
mod models;
mod prelude;
mod state;

use crate::models::WordWithEstimate;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use futures::{StreamExt, TryStreamExt};
use prelude::*;
use serde::Deserialize;
use shared::{
    rusoto_s3::{GetObjectRequest, S3},
    vision::Annotation,
};
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
    .bind("127.0.0.1:8080")
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
    let annotation: Vec<_> = state
        .s3
        .get_object(GetObjectRequest {
            bucket: "newsletter-ocr-na5d".to_string(),
            key: newsletter_id.to_string(),
            ..Default::default()
        })
        .await
        .expect("Cannot read OCR from S3")
        .body
        .unwrap()
        .into_stream()
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .map(|r| r.unwrap())
        .flatten()
        .collect();

    let pwords = serde_json::from_slice::<Annotation>(&annotation)
        .unwrap()
        .words;
    let url = format!(
        "https://newsletter-screenshot-4fj0.s3-eu-west-1.amazonaws.com/{}",
        newsletter_id
    );

    log::trace!("Annotating {} words in {}", pwords.len(), newsletter_id);
    let twords: Vec<_> = pwords.iter().map(|w| &w.word).collect();
    let estimates: Vec<f64> = surf::post("http://127.0.0.1:8888/words")
        .body(serde_json::to_value(&twords).unwrap())
        .await
        .unwrap()
        .take_body()
        .into_json()
        .await
        .unwrap();
    assert_eq!(estimates.len(), pwords.len());

    let words: Vec<_> = pwords
        .into_iter()
        .zip(estimates)
        .map(|(word, estimate)| WordWithEstimate { word, estimate })
        .collect();

    let html = state
        .template
        .render_image_page(&newsletter_id, &url, &words)
        .expect("Cannot render image page");
    HttpResponse::Ok()
        .append_header(("Content-Type", "text/html"))
        .body(html)
}
