use arbitrary::Arbitrary;
use regex::bytes::{Regex, RegexBuilder};
use simplematch::Options;

pub const DEFAULT_ESCAPE: u8 = b'\\';
pub const DEFAULT_WILDCARD_ANY: u8 = b'*';
pub const DEFAULT_WILDCARD_ONE: u8 = b'?';

#[derive(Debug, Clone, Copy, Arbitrary)]
pub struct PatternOptions {
    pub case_sensitive: bool,
    pub is_ranges_enabled: bool,
    pub range_negate: Option<u8>,
    pub wildcard_any: Option<u8>,
    pub wildcard_escape: Option<u8>,
    pub wildcard_one: Option<u8>,
}

impl Default for PatternOptions {
    fn default() -> Self {
        let options = Options::default();
        Self {
            case_sensitive: options.case_sensitive,
            wildcard_escape: options.wildcard_escape,
            is_ranges_enabled: options.is_ranges_enabled,
            range_negate: options.range_negate,
            wildcard_any: options.wildcard_any,
            wildcard_one: options.wildcard_one,
        }
    }
}

impl From<PatternOptions> for Options<u8> {
    fn from(value: PatternOptions) -> Self {
        Self {
            case_sensitive: value.case_sensitive,
            wildcard_escape: value.wildcard_escape,
            is_ranges_enabled: value.is_ranges_enabled,
            range_negate: value.range_negate,
            wildcard_any: value.wildcard_any,
            wildcard_one: value.wildcard_one,
        }
    }
}

pub fn pattern_to_regex(pattern: &str, options: PatternOptions) -> Result<Regex, regex::Error> {
    let PatternOptions {
        case_sensitive,
        wildcard_escape,
        wildcard_any,
        wildcard_one,
        // TODO: Use ranges
        ..
    } = options;

    let wildcard_any = wildcard_any.unwrap_or(DEFAULT_WILDCARD_ANY) as char;
    let wildcard_one = wildcard_one.unwrap_or(DEFAULT_WILDCARD_ONE) as char;
    let (is_escape_enabled, wildcard_escape) = match wildcard_escape {
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
            c if !is_escape && is_escape_enabled && c == wildcard_escape => {
                is_escape = true;
            }
            c if is_escape => {
                if !(c == wildcard_any || c == wildcard_one || c == wildcard_escape) {
                    wrap_char(&mut regex, wildcard_escape)
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
        wrap_char(&mut regex, wildcard_escape);
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
