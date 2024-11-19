use regex::Regex;
use rosepine_build::Config;

trait AsRegex {
    fn to_regex(&self) -> Regex;
}
impl AsRegex for String {
    fn to_regex(&self) -> Regex {
        Regex::new(self).unwrap()
    }
}

pub fn replace_templates(content: String, config: &Config) -> String {
    let template_regex = format!(r#"{}"#, config.prefix).to_regex();
    dbg!(template_regex);
    todo!()
}
