//! The fuzzing library

use arbitrary::Arbitrary;
use quickmatch::{Options, DEFAULT_ESCAPE, DEFAULT_WILDCARD_ANY, DEFAULT_WILDCARD_ONE};
use regex::{Regex, RegexBuilder};

#[derive(Debug, Clone, Copy, Arbitrary)]
pub struct FuzzOptions {
    pub case_sensitive: bool,
    pub escape: Option<u8>,
    pub wildcard_any: Option<u8>,
    pub wildcard_one: Option<u8>,
}

impl Default for FuzzOptions {
    fn default() -> Self {
        let options = Options::default();
        Self {
            case_sensitive: options.case_sensitive,
            escape: options.escape,
            wildcard_any: options.wildcard_any,
            wildcard_one: options.wildcard_one,
        }
    }
}

impl From<FuzzOptions> for Options<u8> {
    fn from(value: FuzzOptions) -> Self {
        Self {
            case_sensitive: value.case_sensitive,
            escape: value.escape,
            wildcard_any: value.wildcard_any,
            wildcard_one: value.wildcard_one,
        }
    }
}

pub fn pattern_to_regex(pattern: &str, options: FuzzOptions) -> Result<Regex, regex::Error> {
    let FuzzOptions {
        case_sensitive,
        escape,
        wildcard_any,
        wildcard_one,
    } = options;

    let wildcard_any = wildcard_any.unwrap_or(DEFAULT_WILDCARD_ANY) as char;
    let wildcard_one = wildcard_one.unwrap_or(DEFAULT_WILDCARD_ONE) as char;
    let (is_escape_enabled, escape) = match escape {
        Some(x) => (true, x as char),
        None => (false, DEFAULT_ESCAPE as char),
    };

    let mut regex = String::with_capacity(pattern.len() * 3);
    regex.push('^');

    let mut is_escape = false;
    for char in pattern.chars() {
        match char {
            c if !is_escape && c == wildcard_any => {
                regex.push_str(".*?");
            }
            c if !is_escape && c == wildcard_one => {
                regex.push('.');
            }
            c if !is_escape && is_escape_enabled && c == escape => {
                is_escape = true;
            }
            c if is_escape => {
                if !(c == wildcard_any || c == wildcard_one || c == escape) {
                    wrap_char(&mut regex, escape)
                }
                wrap_char(&mut regex, c);
                is_escape = false;
            }
            c => {
                wrap_char(&mut regex, c);
            }
        }
    }

    if is_escape {
        wrap_char(&mut regex, escape);
    }
    regex.push('$');

    RegexBuilder::new(&regex)
        .dot_matches_new_line(true)
        .unicode(false)
        .case_insensitive(!case_sensitive)
        .build()
}

fn wrap_char(regex: &mut String, c: char) {
    regex.push('[');
    regex.push(c);
    regex.push(']');
}
