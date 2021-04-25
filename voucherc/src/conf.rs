//! Name of each env var is the same as the property but in ALL_CAPS.

use serde::Deserialize;

#[derive(Default, Deserialize, Debug)]
pub struct Conf {
    pub http_address: String,
}
