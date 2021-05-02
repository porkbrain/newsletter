use serde::Serialize;
use shared::vision::Word;

#[derive(Serialize)]
pub struct WordWithEstimate {
    pub word: Word,
    pub estimate: f64,
    pub is_in_phrase: bool,
}
