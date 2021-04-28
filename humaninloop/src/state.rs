use crate::{html::Template, prelude::*};
use std::sync::Arc;

#[derive(Clone)]
pub struct State {
    pub template: Arc<Template>,
}

impl State {
    pub fn new() -> Result<Self> {
        Ok(Self {
            template: Arc::new(Template::new()?),
        })
    }
}
