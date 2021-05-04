use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Anchor {
    pub href: String,
    pub top: i32,
    pub left: i32,
    pub width: i32,
    pub height: i32,
}
