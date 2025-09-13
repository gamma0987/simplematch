use arbitrary::Arbitrary;
use regex::bytes::{Regex, RegexBuilder};
use simplematch::{Options, Wildcard};

pub const DEFAULT_ESCAPE: u8 = b'\\';
pub const DEFAULT_WILDCARD_ANY: u8 = b'*';
pub const DEFAULT_WILDCARD_ONE: u8 = b'?';
pub const DEFAULT_RANGE_OPEN: u8 = b'[';
pub const DEFAULT_RANGE_NEGATE: u8 = b'!';

/// The `PatternOptions` mimic the `Options` from the main crate
///
/// The main reason for having a separate struct is to avoid `Arbitrary` in the main crate and
/// using `u8` per default. The `Options` in the main crate are generic.
#[derive(Debug, Clone, Copy, Arbitrary)]
pub struct PatternOptions {
    pub case_sensitive: bool,
    pub is_escape_enabled: bool,
    pub is_ranges_enabled: bool,
    pub range_negate: Option<u8>,
    pub wildcard_any: Option<u8>,
    pub wildcard_escape: Option<u8>,
    pub wildcard_one: Option<u8>,
}

impl Default for PatternOptions {
    fn default() -> Self {
        let options = Options::<u8>::default();
        Self {
            case_sensitive: options.case_sensitive,
            is_escape_enabled: options.is_escape_enabled,
            is_ranges_enabled: options.is_classes_enabled,
            range_negate: Some(options.class_negate),
            wildcard_any: Some(options.wildcard_any),
            wildcard_escape: Some(options.wildcard_escape),
            wildcard_one: Some(options.wildcard_one),
        }
    }
}

impl From<PatternOptions> for Options<u8> {
    fn from(value: PatternOptions) -> Self {
        let mut this = Self::default();
        this.case_sensitive = value.case_sensitive;
        this.wildcard_escape = value.wildcard_escape.unwrap_or(u8::DEFAULT_ESCAPE);
        this.is_classes_enabled = value.is_ranges_enabled;
        this.class_negate = value.range_negate.unwrap_or(u8::DEFAULT_CLASS_NEGATE);
        this.wildcard_any = value.wildcard_any.unwrap_or(u8::DEFAULT_ANY);
        this.wildcard_one = value.wildcard_one.unwrap_or(u8::DEFAULT_ONE);
        this.is_escape_enabled = value.is_escape_enabled;

        this
    }
}

/// Convert a wildcard pattern to a regular expression
pub fn pattern_to_regex(pattern: &str, options: PatternOptions) -> Result<Regex, regex::Error> {
    let PatternOptions {
        case_sensitive,
        is_ranges_enabled,
        range_negate,
        wildcard_any,
        wildcard_escape,
        wildcard_one,
        is_escape_enabled,
    } = options;

    let wildcard_any = wildcard_any.unwrap_or(DEFAULT_WILDCARD_ANY) as char;
    let wildcard_one = wildcard_one.unwrap_or(DEFAULT_WILDCARD_ONE) as char;
    let wildcard_escape = wildcard_escape.unwrap_or(DEFAULT_ESCAPE) as char;
    let range_open = DEFAULT_RANGE_OPEN as char;
    let range_negate = range_negate.unwrap_or(DEFAULT_RANGE_NEGATE) as char;

    let mut regex = String::with_capacity(pattern.len() * 3);
    regex.push('^');

    let mut is_escape = false;
    let chars: Vec<char> = pattern.chars().collect();
    let mut index = 0;
    // Besides ranges the parsing is straight forward
    while index < chars.len() {
        match chars[index] {
            c if !is_escape && is_ranges_enabled && c == range_open => {
                // We store ranges in a buffer first which can be discarded if the range is invalid
                let mut range = String::new();
                // We have `[` as first char for sure
                range.push(c);
                index += 1;
                // In case of an invalid range reset to this index
                let range_reset = index;

                // The first char after `[` might be a negation or maybe have reached the end of the
                // pattern.
                if index < chars.len() {
                    if chars[index] == range_negate {
                        range.push('^');
                        index += 1;
                    }
                } else {
                    escape_char(&mut regex, range_open);
                    continue;
                }

                let mut is_valid = false;
                let mut is_first = true;
                while index < chars.len() {
                    let c = chars[index];

                    // The first `]` after `[` (and possibly `!`) is interpreted literally.
                    // Otherwise, we have reached the end of a valid range.
                    if !is_first && c == ']' {
                        range.push(c);
                        is_valid = true;
                        index += 1;
                        break;
                    // We have a range (for example `a-z`)
                    } else if index + 2 < chars.len() && chars[index + 1] == '-' {
                        if chars[index + 2] == ']' {
                            escape_char(&mut range, chars[index]);
                            escape_char(&mut range, chars[index + 1]);
                            range.push(chars[index + 2]);
                            is_valid = true;
                            index += 3;
                            break;
                        } else {
                            // The ranges need to be nested in their own ranges or else multiple
                            // ranges are interpreted wrong by the regex.
                            range.push('[');

                            let start = c;
                            let end = chars[index + 2];

                            let is_special = |c| {
                                c == '-'
                                    || c == '['
                                    || c == ']'
                                    || c == '^'
                                    || c == '|'
                                    || c == '\\'
                                    || c == '.'
                                    || c == '('
                                    || c == ')'
                                    || c == '?'
                                    || c == '*'
                                    || c == '&'
                                    || c == '~'
                                    || c == ':'
                                    || c == '$'
                                    || c == '{'
                                    || c == '}'
                            };

                            // In contrast to our patterns, the regex engine expects the start and
                            // end characters to be ordered from low to high. Also `-` and `]` need
                            // to be escaped properly.
                            if start <= end {
                                escape_char_if(&mut range, start, is_special);
                                range.push('-');
                                escape_char_if(&mut range, end, is_special);
                            } else {
                                escape_char_if(&mut range, end, is_special);
                                range.push('-');
                                escape_char_if(&mut range, start, is_special);
                            }

                            range.push(']');
                            index += 3;
                        }
                    // A single character.
                    } else {
                        escape_char(&mut range, c);
                        index += 1;
                    }

                    is_first = false;
                }

                if is_valid {
                    regex.push_str(&range);
                } else {
                    // In case of an invalid range, we store the `[` and reset to the stored index.
                    escape_char(&mut regex, range_open);
                    index = range_reset;
                }
                continue;
            }
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
                // `]` does not need to be escaped here because it is handled in the ranges parser
                // above or otherwise doesn't have a special meaning if it occurs outside of the
                // context of ranges. `!` and `-` have no special meaning outside of ranges and don't
                // need to be escaped either.
                if !(c == wildcard_any
                    || c == wildcard_one
                    || c == wildcard_escape
                    || c == range_open)
                {
                    wrap_char(&mut regex, wildcard_escape)
                }
                wrap_char(&mut regex, c);
                is_escape = false;
            }
            c => {
                wrap_char(&mut regex, c);
            }
        }

        index += 1;
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

fn escape_char(buffer: &mut String, c: char) {
    let needs_escape = !c.is_ascii_alphanumeric();
    if needs_escape {
        buffer.push('\\');
    }
    buffer.push(c);
}

fn escape_char_if(buffer: &mut String, c: char, predicate: fn(char) -> bool) {
    let needs_escape = predicate(c);
    if needs_escape {
        buffer.push('\\');
    }
    buffer.push(c);
}

fn wrap_char(buffer: &mut String, c: char) {
    buffer.push('[');
    buffer.push(c);
    buffer.push(']');
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    // From fuzzy test
    #[rstest]
    #[case::fuzz_0("*[--$j-/]", Regex::new("^.*?[[\\$-\\-][/-j]]$").unwrap())]
    #[case::fuzz_1("[nzz!aa][bc", Regex::new("^[nzz\\!aa]\\[[b][c]$").unwrap())]
    #[case::fuzz_2("[n", Regex::new("^\\[[n]$").unwrap())]
    #[case::fuzz_3("[.$--.j-.\\/j.]", Regex::new("^[\\.[\\$-\\-]\\.[\\.-j]\\\\\\/j\\.]$").unwrap())]
    #[case::fuzz_5_0("[--$j\0-\0--\0-\0]", Regex::new("^[[\\$-\\-]j[\0-\0][\0-\\-]\\-\\\0]$").unwrap())]
    #[case::fuzz_5_1("*[--$j---\0\0\0\0]", Regex::new("^.*?[[\\$-\\-][\\--j]\\-\\\0\\\0\\\0\\\0]$").unwrap())]
    #[case::fuzz_6("*[--$j---\0\0\0\0]", Regex::new("^.*?[[\\$-\\-][\\--j]\\-\\\0\\\0\\\0\\\0]$").unwrap())]
    #[case::fuzz_7("[\0\0-\0]", Regex::new("^[\\\0[\0-\0]]$").unwrap())]
    #[case::fuzz_8("*[--\nJ-\0\0-\0-+\0]", Regex::new("^.*?[[\n-\\-][\0-J][\0-\0]\\-\\+\\\0]$").unwrap())]
    #[case::fuzz_9("*[-$\0j-/\0a-]", Regex::new("^.*?[\\-\\$\\\0[/-j]\\\0a\\-]$").unwrap())]
    #[case::fuzz_10("[]--]G", Regex::new("^[[\\--\\]]][G]$").unwrap())]
    #[case::fuzz_11("*[]-^\0\0l[]", Regex::new("^.*?[[\\]-\\^]\\\0\\\0l\\[]$").unwrap())]
    fn pattern_to_regex_when_range(#[case] pattern: &str, #[case] expected: Regex) {
        let actual = pattern_to_regex(
            pattern,
            PatternOptions {
                case_sensitive: true,
                is_escape_enabled: false,
                is_ranges_enabled: true,
                range_negate: None,
                wildcard_any: None,
                wildcard_escape: None,
                wildcard_one: None,
            },
        )
        .unwrap();

        assert_eq!(actual.as_str(), expected.as_str());
    }
}
