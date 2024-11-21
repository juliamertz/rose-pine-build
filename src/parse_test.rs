use std::vec;

use crate::{
    config::Config,
    format::Format,
    palette::Role,
    parse::{self, *},
};

#[test]
fn role_variants() {
    assert_capture("$(pine)", Capture::new(vec![Role::Pine], None, None));
    assert_capture(
        "$(rose|love)",
        Capture::new(vec![Role::Rose, Role::Love], None, None),
    );
    assert_capture(
        "$(foam|pine|iris)",
        Capture::new(vec![Role::Foam, Role::Pine, Role::Iris], None, None),
    );
    assert_capture(
        "$(rose|love):hex",
        Capture::new(vec![Role::Rose, Role::Love], Some(Format::Hex), None),
    );
}

#[test]
fn role_variants_whitespace() {
    assert_capture("$( pine )", Capture::new(vec![Role::Pine], None, None));
    assert_capture(
        "$( rose | love )",
        Capture::new(vec![Role::Rose, Role::Love], None, None),
    );
    assert_capture(
        "$( foam | pine | iris )",
        Capture::new(vec![Role::Foam, Role::Pine, Role::Iris], None, None),
    );
    assert_capture(
        "$(    rose   |   love )",
        Capture::new(vec![Role::Rose, Role::Love], None, None),
    );
}

#[test]
fn format() {
    assert_capture(
        "$base:rgb",
        Capture::new(vec![Role::Base], Some(Format::Rgb), None),
    );
    assert_capture(
        "$base:rgb_ns",
        Capture::new(vec![Role::Base], Some(Format::RgbNs), None),
    );
    assert_capture(
        "$base:rgb_function",
        Capture::new(vec![Role::Base], Some(Format::RgbFunction), None),
    );
    assert_capture(
        "$base:rgb_array",
        Capture::new(vec![Role::Base], Some(Format::RgbArray), None),
    );
    assert_capture(
        "$base:rgb_ansi",
        Capture::new(vec![Role::Base], Some(Format::RgbAnsi), None),
    );
    assert_capture(
        "$base:hsl",
        Capture::new(vec![Role::Base], Some(Format::Hsl), None),
    );
    assert_capture(
        "$base:hsl_ns",
        Capture::new(vec![Role::Base], Some(Format::HslNs), None),
    );
    assert_capture(
        "$base:hsl_function",
        Capture::new(vec![Role::Base], Some(Format::HslFunction), None),
    );
    assert_capture(
        "$base:hsl_array",
        Capture::new(vec![Role::Base], Some(Format::HslArray), None),
    );
    assert_capture(
        "$base:hex",
        Capture::new(vec![Role::Base], Some(Format::Hex), None),
    );
    assert_capture(
        "$base:ahex",
        Capture::new(vec![Role::Base], Some(Format::Ahex), None),
    );
    assert_capture(
        "$base:hex_ns",
        Capture::new(vec![Role::Base], Some(Format::HexNs), None),
    );
    assert_capture(
        "$base:ahex_ns",
        Capture::new(vec![Role::Base], Some(Format::AhexNs), None),
    );
}

#[test]
fn opacity() {
    assert_capture("$base/100", Capture::new(vec![Role::Base], None, Some(100)));
    assert_capture("$base/28", Capture::new(vec![Role::Base], None, Some(28)));
    assert_capture("$base/50", Capture::new(vec![Role::Base], None, Some(50)));
    assert_capture("$base/0", Capture::new(vec![Role::Base], None, Some(0)));
    assert_capture(
        "$base:rgb_function/75",
        Capture::new(vec![Role::Base], Some(Format::RgbFunction), Some(75)),
    );
}

fn assert_capture(content: &str, correct: Capture) {
    let config = Config::default();
    let mut lexer = Lexer::new(content, &config);
    match parse::parse_capture(&mut lexer) {
        Ok(mut capture) => {
            // reset positions for testing purposes
            capture.start = 0;
            capture.end = 0;
            assert_eq!(correct, capture)
        }
        Err(e) => {
            panic!("Unable to parse capture, expected: {correct:?}\nerror: {e:?} \nlexer state: {lexer:?}")
        }
    }
}

impl Capture {
    fn new(roles: Vec<Role>, format: Option<Format>, opacity: Option<u16>) -> Self {
        Self {
            role: RoleVariants { roles },
            format,
            opacity,
            start: 0,
            end: 0,
        }
    }
}
