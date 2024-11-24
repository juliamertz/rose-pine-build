use crate::{
    config::Config,
    format::Format,
    palette::Variant,
    parse::{self, Capture},
    utils::Substitutable,
};
use palette::Metadata;
use serde::Serialize;
use strum::IntoEnumIterator;
use tera::{Context, Tera};

#[derive(Clone, Debug, Serialize)]
pub struct Options {
    pub format: Format,
    pub strip_spaces: bool,
    pub force_alpha: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            format: Format::Hex,
            strip_spaces: false,
            force_alpha: false,
        }
    }
}

fn replace_captures(
    captures: Vec<Capture>,
    options: &Options,
    variant: &Variant,
    content: &str,
) -> String {
    let mut buffer: Vec<char> = content.to_owned().chars().collect();
    for capture in captures.into_iter().rev() {
        let role = &capture.format(variant, options);
        buffer.substitute(&role.chars().collect(), capture.start, capture.end);
    }

    buffer.into_iter().collect()
}

pub fn generate_variant(variant: &Variant, config: &Config, content: &str) -> String {
    let captures = parse::parse_template(content, config);
    replace_captures(
        captures.into_iter().flatten().collect(),
        &config.generate,
        variant,
        content,
    )
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

pub fn render_template(variant: &Variant, template: &str) -> tera::Result<String> {
    Tera::one_off(template, &create_context(variant), true)
}

pub fn render_templates(template: &str) -> tera::Result<Vec<(Variant, String)>> {
    let mut result = vec![];

    for variant in Variant::iter() {
        result.push((
            variant,
            Tera::one_off(template, &create_context(&variant), true)?,
        ))
    }

    Ok(result)
}

pub fn generate_variants(config: &Config, content: &str) -> Vec<(Variant, String)> {
    Variant::iter()
        .map(|v| (v, generate_variant(&v, config, content)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_rgb() {
        assert_eq!(
            generate_variant(Variant::Moon, "$love:rgb"),
            "235, 111, 146"
        );
        assert_eq!(
            generate_variant(Variant::Moon, "$love:rgb_function"),
            "rgb(235, 111, 146)"
        );
        assert_eq!(
            generate_variant(Variant::Moon, "$pine:rgb_function/80"),
            "rgb(62, 143, 176, 0.8)"
        );
    }

    #[test]
    fn format_parse_order() {
        assert_eq!(
            generate_variant(
                Variant::Moon,
                "$love:rgb_function; $love:rgb; $love:hex_ns; $love:hex",
            ),
            "rgb(235, 111, 146); 235, 111, 146; eb6f92; #eb6f92"
        );
    }

    #[test]
    fn format_hsl() {
        assert_eq!(
            generate_variant(Variant::Moon, "$love:hsl_function"),
            "hsl(343, 76%, 68%)"
        );
    }

    #[test]
    fn opacity() {
        assert_eq!(
            generate_variant(Variant::Moon, "$love:rgb_function/50"),
            "rgb(235, 111, 146, 0.5)"
        );
        assert_eq!(
            generate_variant(Variant::Moon, "$love:hsl_function/50"),
            "hsl(343, 76%, 68%, 0.5)"
        );
        assert_eq!(
            generate_variant(Variant::Moon, "$love:hex/100"),
            "#eb6f92ff"
        );
        assert_eq!(generate_variant(Variant::Moon, "$love:hex/0"), "#eb6f9200");
        assert_eq!(
            generate_variant(Variant::Moon, "$love:ahex_ns/50"),
            "80eb6f92"
        );
        assert_eq!(
            generate_variant(Variant::Moon, "$love:ahex_ns/100"),
            "ffeb6f92"
        );
    }

    #[test]
    fn role_variation() {
        assert_eq!(generate_variant(Variant::Main, "$(pine|foam)"), "#31748f");
        assert_eq!(
            generate_variant(Variant::Main, "$(rose|love):hex"),
            "#ebbcba"
        );
        assert_eq!(
            generate_variant(Variant::Dawn, "$(rose|love):hex"),
            "#b4637a"
        );
    }

    fn generate_variant(variant: Variant, content: &str) -> String {
        super::generate_variant(&variant, &Config::default(), content)
    }
}
