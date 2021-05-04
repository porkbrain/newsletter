use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Anchor {
    pub href: String,
    pub top: isize,
    pub left: isize,
    pub width: isize,
    pub height: isize,
}
