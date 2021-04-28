use crate::{models::WordWithEstimate, prelude::*};
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
        handlebars.register_helper(
            "certainty-colour",
            Box::new(helpers::certainty_colour),
        );

        Ok(Self(handlebars))
    }

    pub fn render_image_page(
        &self,
        id: &str,
        url: &str,
        words: &[WordWithEstimate],
    ) -> Result<String> {
        log::trace!("Rendering image page");
        self.render(
            IMAGE_TEMPLATE_NAME,
            &json!({ "id": id, "url": url, "words": words }),
        )
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

    /// Converts [0; 1] certainty number to a colour which can be used for
    /// background
    pub fn certainty_colour(
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let c = h.param(0).unwrap().value().as_f64().unwrap();
        // light green
        out.write(&format!("rgba(255, 255, 0, {:.2})", (c - 0.6).max(0.0)))?;
        Ok(())
    }
}
