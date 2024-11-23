use super::*;
use crate::parse;

#[test]
fn role_variants() {
    assert_role("$(pine)", vec![Role::Pine], None, None);
    assert_role("$(rose|love)", vec![Role::Rose, Role::Love], None, None);
    assert_role(
        "$(foam|pine|iris)",
        vec![Role::Foam, Role::Pine, Role::Iris],
        None,
        None,
    );
    assert_role(
        "$(rose|love):hex",
        vec![Role::Rose, Role::Love],
        Some(Format::Hex),
        None,
    );
}

#[test]
fn role_variants_whitespace() {
    assert_role("$( pine )", vec![Role::Pine], None, None);
    assert_role("$( rose | love )", vec![Role::Rose, Role::Love], None, None);
    assert_role(
        "$( foam | pine | iris )",
        vec![Role::Foam, Role::Pine, Role::Iris],
        None,
        None,
    );
    assert_role(
        "$(    rose   |   love )",
        vec![Role::Rose, Role::Love],
        None,
        None,
    );
}

#[test]
fn format() {
    assert_role("$base:rgb", vec![Role::Base], Some(Format::Rgb), None);
    assert_role("$base:rgb_ns", vec![Role::Base], Some(Format::RgbNs), None);
    assert_role(
        "$base:rgb_function",
        vec![Role::Base],
        Some(Format::RgbFunction),
        None,
    );
    assert_role(
        "$base:rgb_array",
        vec![Role::Base],
        Some(Format::RgbArray),
        None,
    );
    assert_role(
        "$base:rgb_ansi",
        vec![Role::Base],
        Some(Format::RgbAnsi),
        None,
    );
    assert_role("$base:hsl", vec![Role::Base], Some(Format::Hsl), None);
    assert_role("$base:hsl_ns", vec![Role::Base], Some(Format::HslNs), None);
    assert_role(
        "$base:hsl_function",
        vec![Role::Base],
        Some(Format::HslFunction),
        None,
    );
    assert_role(
        "$base:hsl_array",
        vec![Role::Base],
        Some(Format::HslArray),
        None,
    );
    assert_role("$base:hex", vec![Role::Base], Some(Format::Hex), None);
    assert_role("$base:ahex", vec![Role::Base], Some(Format::Ahex), None);
    assert_capture(
        "$base:hex_ns",
        Template::Role(RoleCaptures(vec![Role::Base]), Some(Format::HexNs), None),
    );
    assert_capture(
        "$base:ahex_ns",
        Template::Role(RoleCaptures(vec![Role::Base]), Some(Format::AhexNs), None),
    );
}

#[test]
fn opacity() {
    assert_role("$base/100", vec![Role::Base], None, Some(100));
    assert_role("$base/28", vec![Role::Base], None, Some(28));
    assert_role("$base/50", vec![Role::Base], None, Some(50));
    assert_role("$base/0", vec![Role::Base], None, Some(0));
    assert_role(
        "$base:rgb_function/75",
        vec![Role::Base],
        Some(Format::RgbFunction),
        Some(75),
    );
}

#[test]
fn metadata() {
    assert_metadata("$name", MetadataKey::Name, None);
    assert_metadata("$name:title", MetadataKey::Name, Some(Case::Title));
}

fn assert_role(content: &str, roles: Vec<Role>, format: Option<Format>, alpha: Option<u16>) {
    assert_capture(content, Template::Role(RoleCaptures(roles), format, alpha));
}

fn assert_metadata(content: &str, key: MetadataKey, case: Option<Case>) {
    assert_capture(content, Template::Metadata(key, case));
}

fn assert_capture(content: &str, correct: Template) {
    let config = Config::default();
    let mut lexer = Lexer::new(content, &config);
    match parse::parse_capture(&mut lexer) {
        Ok(mut capture) => {
            // reset positions for testing purposes
            capture.start = 0;
            capture.end = 0;
            assert_eq!(correct, capture.template)
        }
        Err(e) => {
            panic!("Unable to parse capture, expected: {correct:?}\nerror: {e:?} \nlexer state: {lexer:?}")
        }
    }
}
