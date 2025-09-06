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
    pub const fn case_insensitive(self, yes: bool) -> Self {
        Self {
            case_sensitive: !yes,
            ..self
        }
    }

    #[must_use]
    pub const fn enable_escape(self) -> Self {
        Self {
            escape: Some(DEFAULT_ESCAPE),
            ..self
        }
    }

    #[must_use]
    pub const fn enable_escape_with(self, byte: u8) -> Self {
        Self {
            escape: Some(byte),
            ..self
        }
    }

    #[must_use]
    pub const fn wildcard_any_with(self, byte: u8) -> Self {
        Self {
            wildcard_any: Some(byte),
            ..self
        }
    }

    #[must_use]
    pub const fn wildcard_one_with(self, byte: u8) -> Self {
        Self {
            wildcard_one: Some(byte),
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
/// patterns the speedup can be even higher.
#[must_use]
pub fn dowild<P, H>(pattern: P, haystack: H) -> bool
where
    P: AsRef<[u8]>,
    H: AsRef<[u8]>,
{
    dowild_bytes(pattern.as_ref(), haystack.as_ref())
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
