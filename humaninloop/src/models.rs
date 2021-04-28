use serde::Serialize;
use shared::vision::Word;

#[derive(Serialize)]
pub struct WordWithEstimate {
    pub word: Word,
    pub estimate: f64,
}
