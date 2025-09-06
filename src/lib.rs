//! The library

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// TODO: Remove
#![allow(missing_docs)]

pub const DEFAULT_ESCAPE: u8 = b'\\';
pub const DEFAULT_WILDCARD_ANY: u8 = b'*';
pub const DEFAULT_WILDCARD_ONE: u8 = b'?';

pub trait QuickMatch {
    #[must_use]
    fn dowild(&self, pattern: &str) -> bool;
    #[must_use]
    fn dowild_with(&self, pattern: &str, options: Options) -> bool;
}

pub trait QuickMatchBytes {
    #[must_use]
    fn dowild(&self, pattern: &[u8]) -> bool;
    #[must_use]
    fn dowild_with(&self, pattern: &[u8], options: Options) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "_fuzz", derive(arbitrary::Arbitrary))]
pub struct Options {
    pub case_sensitive: bool,
    pub escape: Option<u8>,
    pub wildcard_any: Option<u8>,
    pub wildcard_one: Option<u8>,
}

impl Default for Options {
    fn default() -> Self {
        Self::new()
    }
}

impl Options {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            case_sensitive: true,
            escape: None,
            wildcard_any: Some(b'*'),
            wildcard_one: Some(b'?'),
        }
    }

    #[must_use]
    pub const fn case_insensitive(self) -> Self {
        Self {
            case_sensitive: false,
            ..self
        }
    }

    #[must_use]
    pub const fn enable_escape(self, byte: u8) -> Self {
        Self {
            escape: Some(byte),
            ..self
        }
    }
}

impl QuickMatch for &str {
    fn dowild(&self, pattern: &str) -> bool {
        dowild(pattern, self)
    }

    fn dowild_with(&self, pattern: &str, options: Options) -> bool {
        dowild_with(pattern, self, options)
    }
}

impl QuickMatchBytes for &[u8] {
    fn dowild(&self, pattern: &[u8]) -> bool {
        dowild_bytes(pattern, self)
    }

    fn dowild_with(&self, pattern: &[u8], options: Options) -> bool {
        dowild_bytes_with(pattern, self, options)
    }
}

#[cfg(feature = "std")]
impl QuickMatchBytes for Vec<u8> {
    fn dowild(&self, pattern: &[u8]) -> bool {
        dowild_bytes(pattern, self)
    }

    fn dowild_with(&self, pattern: &[u8], options: Options) -> bool {
        dowild_bytes_with(pattern, self, options)
    }
}

#[cfg(feature = "std")]
impl QuickMatch for String {
    fn dowild(&self, pattern: &str) -> bool {
        self.as_str().dowild(pattern)
    }

    fn dowild_with(&self, pattern: &str, options: Options) -> bool {
        self.as_str().dowild_with(pattern, options)
    }
}

/// Return true if the wildcard pattern matches the `haystack`
///
/// Allowed wildcard characters are `*` to match any amount of characters and `?` to match
/// exactly one character.
///
/// TODO: Escaping is supported.
///
/// # Examples
///
/// TODO: examples
///
/// # Credits
///
/// This linear-time glob algorithm is based on the algorithm from the article
/// <https://research.swtch.com/glob> written by Russ Cox and was further improved here.
///
/// The improved version uses generally about 2-5x less instructions. For "normal" and short
/// patterns the speedup can be even more, up to 6-7x.
#[must_use]
pub fn dowild<P, H>(pattern: P, haystack: H) -> bool
where
    P: AsRef<str>,
    H: AsRef<str>,
{
    dowild_bytes(pattern.as_ref().as_bytes(), haystack.as_ref().as_bytes())
}

#[must_use]
pub const fn dowild_bytes(pattern: &[u8], haystack: &[u8]) -> bool {
    let mut p_idx = 0;
    let mut h_idx = 0;

    let mut next_p_idx = 0;
    let mut next_h_idx = 0;

    while p_idx < pattern.len() || h_idx < haystack.len() {
        if p_idx < pattern.len() {
            match pattern[p_idx] {
                b'*' => {
                    next_p_idx = p_idx;
                    p_idx += 1;

                    while p_idx < pattern.len() && pattern[p_idx] == b'*' {
                        p_idx += 1;
                    }
                    if p_idx >= pattern.len() {
                        return true;
                    }

                    let c = pattern[p_idx];

                    // In this special case, the compiler seems to optimize the else branch far
                    // better with both branches explicitly having the same increment at the end.
                    #[allow(clippy::branches_sharing_code)]
                    if c == b'?' {
                        next_h_idx = h_idx + 1;
                    } else {
                        // Advancing the haystack to the first match significantly enhances the speed
                        // compared to the original algorithm.
                        while h_idx < haystack.len() && haystack[h_idx] != c {
                            h_idx += 1;
                        }
                        next_h_idx = h_idx + 1;
                    }

                    continue;
                }
                b'?' => {
                    if h_idx < haystack.len() {
                        p_idx += 1;
                        h_idx += 1;
                        continue;
                    }
                }
                c => {
                    if h_idx < haystack.len() && haystack[h_idx] == c {
                        p_idx += 1;
                        h_idx += 1;
                        continue;
                    }
                }
            }
        }
        if 0 < next_h_idx && next_h_idx <= haystack.len() {
            p_idx = next_p_idx;
            h_idx = next_h_idx;
            continue;
        }
        return false;
    }
    true
}

#[must_use]
pub fn dowild_with<P, H>(pattern: P, haystack: H, options: Options) -> bool
where
    P: AsRef<[u8]>,
    H: AsRef<[u8]>,
{
    dowild_bytes_with(pattern.as_ref(), haystack.as_ref(), options)
}

#[must_use]
pub const fn dowild_bytes_with(pattern: &[u8], haystack: &[u8], options: Options) -> bool {
    let Options {
        case_sensitive,
        escape,
        wildcard_any,
        wildcard_one,
    } = options;

    let wildcard_any = match wildcard_any {
        Some(x) => x,
        None => DEFAULT_WILDCARD_ANY,
    };
    let wildcard_one = match wildcard_one {
        Some(x) => x,
        None => DEFAULT_WILDCARD_ONE,
    };
    let (is_escape_enabled, escape) = match escape {
        Some(x) => (true, x),
        // although the `DEFAULT_ESCAPE` is not used but we need to assign
        // some reasonable value
        None => (false, DEFAULT_ESCAPE),
    };

    let mut p_idx = 0;
    let mut h_idx = 0;

    let mut next_p_idx = 0;
    let mut next_h_idx = 0;

    while p_idx < pattern.len() || h_idx < haystack.len() {
        if p_idx < pattern.len() {
            match pattern[p_idx] {
                c if c == wildcard_any => {
                    next_p_idx = p_idx;
                    p_idx += 1;

                    while p_idx < pattern.len() && pattern[p_idx] == wildcard_any {
                        p_idx += 1;
                    }
                    if p_idx >= pattern.len() {
                        return true;
                    }

                    let c = pattern[p_idx];

                    #[allow(clippy::branches_sharing_code)]
                    if c == wildcard_one || (is_escape_enabled && c == escape) {
                        next_h_idx = h_idx + 1;
                    } else {
                        while h_idx < haystack.len() && haystack[h_idx] != c {
                            h_idx += 1;
                        }
                        next_h_idx = h_idx + 1;
                    }

                    continue;
                }
                c if c == wildcard_one => {
                    if h_idx < haystack.len() {
                        p_idx += 1;
                        h_idx += 1;
                        continue;
                    }
                }
                c if is_escape_enabled && c == escape && p_idx + 1 < pattern.len() => {
                    if h_idx < haystack.len() {
                        let c = pattern[p_idx + 1];
                        let h = haystack[h_idx];

                        let is_special = c == wildcard_any || c == wildcard_one || c == escape;
                        #[allow(clippy::else_if_without_else)]
                        if is_special && h == c {
                            p_idx += 2;
                            h_idx += 1;
                            continue;
                        } else if !is_special && h == escape {
                            p_idx += 1;
                            h_idx += 1;
                            continue;
                        }
                    }
                }
                c => {
                    if h_idx < haystack.len() && match_case(haystack[h_idx], c, case_sensitive) {
                        p_idx += 1;
                        h_idx += 1;
                        continue;
                    }
                }
            }
        }
        if 0 < next_h_idx && next_h_idx <= haystack.len() {
            p_idx = next_p_idx;
            h_idx = next_h_idx;
            continue;
        }
        return false;
    }
    true
}

#[inline]
const fn match_case(byte: u8, other: u8, case_sensitive: bool) -> bool {
    if case_sensitive {
        byte == other
    } else {
        byte.eq_ignore_ascii_case(&other)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    // These are mostly the tests from the original algorithm
    // cspell: disable
    #[rstest]
    #[case::empty("", "", true)]
    #[case::one_and_empty("x", "", false)]
    #[case::empty_and_one("", "x", false)]
    #[case::simple("abc", "abc", true)]
    #[case::star("*", "abc", true)]
    #[case::star_c("*c", "abc", true)]
    #[case::star_b("*b", "abc", false)]
    #[case::a_star("a*", "abc", true)]
    #[case::b_star("b*", "abc", false)]
    #[case::a_star_single("a*", "a", true)]
    #[case::star_a_single("*a", "a", true)]
    #[case::multi_star_no_end("a*b*c*d*e*", "axbxcxdxe", true)]
    #[case::multi_star_end("a*b*c*d*e*", "axbxcxdxexxx", true)]
    #[case::star_and_question_mark_0("a*b?c*x", "abxbbxdbxebxczzx", true)]
    #[case::star_and_question_mark_1("a*b?c*x", "abxbbxdbxebxczzy", false)]
    #[case::for_debug("a*?b", format!("{}b", "a".repeat(100)), true)]
    #[case::for_debug("a*?b", "aab", true)]
    #[case::multi_a_star("a*a*a*a*b", "a".repeat(100), false)]
    #[case::star_x("*x", "xxx", true)]
    // cspell: enable
    fn simple_dowild(#[case] pattern: String, #[case] haystack: String, #[case] expected: bool) {
        assert_eq!(dowild(pattern.as_str(), haystack.as_str()), expected);
        assert_eq!(
            dowild_with(
                pattern.as_str(),
                haystack.as_str(),
                Options::new().enable_escape(DEFAULT_ESCAPE)
            ),
            expected
        );
        assert_eq!(
            dowild_with(
                pattern.as_str(),
                haystack.as_str(),
                Options::new()
                    .case_insensitive()
                    .enable_escape(DEFAULT_ESCAPE)
            ),
            expected
        );
        assert_eq!(
            dowild_with(pattern.as_str(), haystack.as_str(), Options::new()),
            expected
        );
        assert_eq!(
            dowild_with(
                pattern.as_str(),
                haystack.as_str(),
                Options::new().case_insensitive()
            ),
            expected
        );
    }

    #[test]
    fn impl_for_str() {
        assert_eq!("abc".dowild("a*c"), true);
        assert_eq!(
            "abc".dowild_with("a*c", Options::new().enable_escape(DEFAULT_ESCAPE)),
            true
        );
    }

    #[test]
    fn impl_for_string() {
        assert_eq!(String::from("abc").dowild("a*c"), true);
        assert_eq!(
            String::from("abc").dowild_with("a*c", Options::new().enable_escape(DEFAULT_ESCAPE)),
            true
        );
    }

    #[rstest]
    #[case::escape_match_self("\\", "\\", true)]
    #[case::double_escape_match_self("\\\\", "\\", true)]
    #[case::triple_escape_match_self("\\\\\\", "\\\\", true)]
    #[case::match_star("\\*", "*", true)]
    #[case::match_question_mark("\\?", "?", true)]
    #[case::a_star("a\\*", "a*", true)]
    #[case::a_star_star("a\\**", "a*xxx", true)]
    #[case::escape_non_escape("\\a", "\\a", true)]
    fn dowild_with_default_escape(
        #[case] pattern: String,
        #[case] haystack: String,
        #[case] expected: bool,
    ) {
        let options = Options::new().enable_escape(DEFAULT_ESCAPE);
        assert_eq!(
            dowild_with(pattern.as_str(), haystack.as_str(), options),
            expected
        );
    }

    #[rstest]
    #[case::escape_true("\0\0\0*\0", "\0*\0", true)]
    #[case::escape_star_then_false("\0\0\0*\0", "\0*\0\0\0\0\0\0\0", false)]
    fn dowild_with_custom_escape(
        #[case] pattern: String,
        #[case] haystack: String,
        #[case] expected: bool,
    ) {
        let options = Options::new().enable_escape(b'\0');
        assert_eq!(
            dowild_with(pattern.as_str(), haystack.as_str(), options),
            expected
        );
    }
}
