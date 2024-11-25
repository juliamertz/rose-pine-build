use palette::{Metadata, Variant};
use strum::IntoEnumIterator;
use anyhow::Result;
use tera::{Context, Tera};

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
    // let context = create_context(variant);
    let mut tera = Tera::default();
    tera.add_raw_template("content", &template)?;

    // TODO:
    Ok(Variant::iter()
        .map(|v| (v, tera.render("content", &create_context(&v)).unwrap()))
        .collect())
}

// pub fn generate_templates(templates: Vec<String>, variant: &Variant) -> Result<Vec<String>> {
//     let context = create_context(variant);
//     templates
//         .into_iter()
//         .map(|t| Tera::one_off(&t, &context, true))
//         .collect()
// }
