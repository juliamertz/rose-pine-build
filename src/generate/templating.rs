use anyhow::Result;
use palette::{Metadata, Variant};
use strum::IntoEnumIterator;
use tera::{Context, Tera};

mod filters {
    use std::collections::HashMap;

    pub fn trunc(
        value: &tera::Value,
        args: &HashMap<String, tera::Value>,
    ) -> Result<tera::Value, tera::Error> {
        let value: f64 = tera::from_value(value.clone())?;
        let places: usize = tera::from_value(
            args.get("places")
                .ok_or_else(|| tera::Error::msg("number of places is required"))?
                .clone(),
        )?;
        Ok(tera::to_value(format!("{value:.places$}"))?)
    }
}

fn create_context(variant: &Variant) -> Context {
    let mut ctx = Context::new();
    let meta: Metadata = variant.into();
    ctx.insert("metadata", &meta);
    for (role, color) in variant.colors() {
        ctx.insert(role, &color);
    }

    ctx
}

pub fn generate_variants(template: String) -> Result<Vec<(Variant, String)>> {
    let mut tera = Tera::default();
    tera.register_filter("trunc", filters::trunc);
    tera.add_raw_template("content", &template)?;

    // TODO:
    Ok(Variant::iter()
        .map(|v| (v, tera.render("content", &create_context(&v)).unwrap()))
        .collect())
}
