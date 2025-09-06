//! The library

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// TODO: Remove
#![allow(missing_docs)]

macro_rules! impl_quickmatch {
    ( $type:ty: $for:ty ) => {
        impl QuickMatch<$type> for $for {
            fn dowild(&self, pattern: Self) -> bool {
                dowild(pattern, self)
            }

            fn dowild_with(&self, pattern: Self, options: Options<$type>) -> bool {
                dowild_with(pattern, self, options)
            }
        }
    };
    ( $type:ty: $for:ty => $( $tail:tt )* ) => {
        impl QuickMatch<$type> for $for {
            fn dowild(&self, pattern: Self) -> bool {
                dowild(pattern $( $tail )*, self $( $tail )* )
            }

            fn dowild_with(&self, pattern: Self, options: Options<$type>) -> bool {
                dowild_with(pattern $( $tail )*, self $( $tail )*, options)
            }
        }
    };
}

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::string::String;
#[cfg(feature = "std")]
use std::vec::Vec;

pub const DEFAULT_ESCAPE: u8 = b'\\';
pub const DEFAULT_WILDCARD_ANY: u8 = b'*';
pub const DEFAULT_WILDCARD_ONE: u8 = b'?';

pub trait QuickMatch<T>
where
    T: Wildcard,
{
    #[must_use]
    fn dowild(&self, pattern: Self) -> bool;
    #[must_use]
    fn dowild_with(&self, pattern: Self, options: Options<T>) -> bool;
}

pub trait Wildcard: Eq + Copy {
    const DEFAULT_ANY: Self;
    const DEFAULT_ESCAPE: Self;
    const DEFAULT_ONE: Self;

    fn match_case(first: Self, second: Self, case_sensitive: bool) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Options<T>
where
    T: Wildcard,
{
    pub case_sensitive: bool,
    pub escape: Option<T>,
    pub wildcard_any: Option<T>,
    pub wildcard_one: Option<T>,
}

impl<T> Default for Options<T>
where
    T: Wildcard,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Options<T>
where
    T: Wildcard,
{
    #[must_use]
    pub const fn new() -> Self {
        Self {
            case_sensitive: true,
            escape: None,
            wildcard_any: Some(T::DEFAULT_ANY),
            wildcard_one: Some(T::DEFAULT_ONE),
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
            escape: Some(T::DEFAULT_ESCAPE),
            ..self
        }
    }

    #[must_use]
    pub const fn enable_escape_with(self, token: T) -> Self {
        Self {
            escape: Some(token),
            ..self
        }
    }

    #[must_use]
    pub const fn wildcard_any_with(self, token: T) -> Self {
        Self {
            wildcard_any: Some(token),
            ..self
        }
    }

    #[must_use]
    pub const fn wildcard_one_with(self, token: T) -> Self {
        Self {
            wildcard_one: Some(token),
            ..self
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Our trait implementations for the basic types
////////////////////////////////////////////////////////////////////////////////

impl_quickmatch!(u8: &[u8]);
impl_quickmatch!(u8: &str => .as_bytes());
impl_quickmatch!(u8: String => .as_bytes());
impl_quickmatch!(u8: Vec<u8> => .as_slice());
impl_quickmatch!(char: &[char]);
impl_quickmatch!(char: Vec<char> => .as_slice());

impl Wildcard for u8 {
    const DEFAULT_ANY: Self = b'*';
    const DEFAULT_ESCAPE: Self = b'\\';
    const DEFAULT_ONE: Self = b'?';

    fn match_case(first: Self, second: Self, case_sensitive: bool) -> bool {
        if case_sensitive {
            first == second
        } else {
            first.eq_ignore_ascii_case(&second)
        }
    }
}

impl Wildcard for char {
    const DEFAULT_ANY: Self = '*';
    const DEFAULT_ESCAPE: Self = '\\';
    const DEFAULT_ONE: Self = '?';

    fn match_case(first: Self, second: Self, case_sensitive: bool) -> bool {
        if case_sensitive {
            first == second
        } else {
            first.to_lowercase().eq(second.to_lowercase())
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// The main dowild functions
////////////////////////////////////////////////////////////////////////////////

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
pub fn dowild<T>(pattern: &[T], haystack: &[T]) -> bool
where
    T: Wildcard,
{
    let mut p_idx = 0;
    let mut h_idx = 0;

    let mut next_p_idx = 0;
    let mut next_h_idx = 0;

    let wildcard_any = T::DEFAULT_ANY;
    let wildcard_one = T::DEFAULT_ONE;

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

                    // In this special case, the compiler seems to optimize the else branch far
                    // better with both branches explicitly having the same increment at the end.
                    #[allow(clippy::branches_sharing_code)]
                    if c == wildcard_one {
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
                c if c == wildcard_one => {
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
pub fn dowild_with<T>(pattern: &[T], haystack: &[T], options: Options<T>) -> bool
where
    T: Wildcard,
{
    let Options {
        case_sensitive,
        escape,
        wildcard_any,
        wildcard_one,
    } = options;

    let wildcard_any = wildcard_any.unwrap_or(T::DEFAULT_ANY);
    let wildcard_one = wildcard_one.unwrap_or(T::DEFAULT_ONE);
    let (is_escape_enabled, escape) = match escape {
        Some(x) => (true, x),
        // although the value for `escape` is not used we need to assign some reasonable value
        None => (false, T::DEFAULT_ESCAPE),
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
                    if h_idx < haystack.len() && T::match_case(haystack[h_idx], c, case_sensitive) {
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
