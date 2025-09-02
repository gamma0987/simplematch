//! The fuzzing library

use quickmatch::{Options, DEFAULT_ESCAPE, DEFAULT_WILDCARD_ANY, DEFAULT_WILDCARD_ONE};
use regex::{Regex, RegexBuilder};

pub fn pattern_to_regex(pattern: &str, options: Options) -> Result<Regex, regex::Error> {
    let Options {
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

    let wrap_char = |regex: &mut String, c| {
        regex.push('[');
        regex.push(c);
        regex.push(']');
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
                if !(c == '*' || c == '?' || c == '\\') && c != escape {
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
