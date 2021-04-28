use crate::prelude::*;
use handlebars::Handlebars;
use serde_json::{json, Value};

const IMAGE_TEMPLATE_CONTENTS: &str =
    include_str!("assets/image.handlebars.html");

const IMAGE_TEMPLATE_NAME: &str = "image";

pub struct Template(Handlebars<'static>);

impl Template {
    pub fn new() -> Result<Self> {
        let mut handlebars = Handlebars::new();

        handlebars.register_template_string(
            IMAGE_TEMPLATE_NAME,
            IMAGE_TEMPLATE_CONTENTS,
        )?;

        handlebars.register_helper("minus", Box::new(helpers::minus));
        handlebars.register_helper("hash", Box::new(helpers::hash));

        Ok(Self(handlebars))
    }

    pub fn render_image_page(
        &self,
        url: &str,
        words: &Value,
    ) -> Result<String> {
        self.render(IMAGE_TEMPLATE_NAME, &json!({ "url": url, "words": words }))
    }

    fn render(&self, template: &str, json: &Value) -> Result<String> {
        let html = self.0.render(template, &json)?;
        Ok(html)
    }
}

mod helpers {
    use handlebars::{
        Context, Handlebars, Helper, HelperResult, Output, RenderContext,
    };
    use shared::vision::Word;
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    pub fn minus(
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let subtrahend = h.param(0).unwrap().value().as_u64().unwrap();
        let minuend = h.param(1).unwrap().value().as_u64().unwrap();

        out.write(&format!("{}", subtrahend - minuend))?;
        Ok(())
    }

    pub fn hash(
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let word: Word =
            serde_json::from_value(h.param(0).unwrap().value().clone())
                .unwrap();

        let mut hasher = DefaultHasher::new();
        word.hash(&mut hasher);
        let hash = hasher.finish();

        out.write(&format!("{}", hash))?;
        Ok(())
    }
}
