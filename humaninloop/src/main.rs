mod html;
mod prelude;
mod state;

use dotenv::dotenv;
use prelude::*;
use state::State;
use tide::{Request, Response};

#[async_std::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    log::info!("Starting humaninloop v{}", env!("CARGO_PKG_VERSION"));

    let mut app = tide::with_state(State::new()?);
    app.at("/").get(show_image);
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

async fn show_image(req: Request<State>) -> tide::Result {
    let ocr: serde_json::Value =
        serde_json::from_str(include_str!("../../clis/test.json"))?;
    let words = ocr.get("words").unwrap();
    let url =
    "https://newsletter-screenshot-4fj0.s3-eu-west-1.amazonaws.com/00n62c957vc2u4vgqvc1p2ffemad13qfehcsjto1";

    match req.state().template.render_image_page(url, words) {
        Ok(html) => Ok(Response::builder(200)
            .header("Content-Type", "text/html")
            .body(html)
            .build()),
        Err(e) => {
            log::error!("Cannot render image page due to {}", e);
            Err(tide::Error::from_str(500, "Cannot render image page"))
        }
    }
}
