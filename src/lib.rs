//! # simplematch
//!
//! The `simplematch` library provides a fast and efficient way to match wildcard patterns on
//! strings and bytes. It includes two primary functions, `dowild` and `dowild_with`, along
//! with an `Options` struct to customize the behavior of the `dowild_with` function.
//!
//! ## Usage
//!
//! To use the `simplematch` library, include it in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! simplematch = "0.1"
//! ```
//!
//! ## Functions
//!
//! ### `dowild`
//!
//! This function is the most performant but has no customization options.
//!
//! ```rust, ignore
//! pub fn dowild<T>(pattern: &[T], haystack: &[T]) -> bool
//! where
//!     T: Wildcard
//! ```
//!
//! `Wildcard` is natively implemented for `u8` and `char`.
//!
//! Matches the given `haystack` against the specified `pattern` using simple wildcard rules.
//! The `*` character matches any sequence of characters, while the `?` character matches
//! a single character.
//!
//! **Parameters:**
//! - `pattern`: A bytes or char slice representing the wildcard pattern to match against.
//! - `haystack`: A bytes or char slice representing the text to be matched.
//!
//! **Returns:**
//! - `true` if the `pattern` matches the `haystack`, otherwise `false`.
//!
//! #### Examples
//!
//! ```rust
//! use simplematch::dowild;
//!
//! assert_eq!(dowild("foo*".as_bytes(), "foobar".as_bytes()), true);
//! assert_eq!(dowild("foo?".as_bytes(), "fooa".as_bytes()), true)
//! ```
//!
//! Or, bringing the trait [`SimpleMatch`] in scope allows for more convenient access to this
//! function without performance loss:
//!
//! ```rust
//! use simplematch::SimpleMatch;
//!
//! assert_eq!("foobar".dowild("foo*"), true);
//! ```
//!
//! A possible usage with `char`:
//!
//! ```rust
//! use simplematch::SimpleMatch;
//!
//! let pattern = "foo*".chars().collect::<Vec<char>>();
//! let haystack = "foobar".chars().collect::<Vec<char>>();
//!
//! assert_eq!(haystack.dowild(pattern), true);
//! ```
//!
//! ### `dowild_with`
//!
//! ```rust, ignore
//! use simplematch::Options;
//!
//! pub fn dowild_with<T>(pattern: &[T], haystack: &[T], options: Options<T>) -> bool
//! where
//!    T: Wildcard + Ord,
//! ```
//!
//! Matches the given `haystack` against the specified `pattern` with customizable options.
//! This function allows for matching case insensitive, custom wildcard characters, escaping
//! special characters and character classes including ranges.
//!
//! **Parameters:**
//! - `pattern`: A bytes or char slice representing the wildcard pattern to match against.
//! - `haystack`: A bytes or char slice representing the text to be matched.
//! - `options`: An instance of the [`Options`] struct to customize the matching behavior.
//!
//! **Returns:**
//! - `true` if the `pattern` matches the `haystack` according to the specified options,
//!   otherwise `false`.
//!
//! #### Examples
//!
//! ```rust
//! use simplematch::{dowild_with, Options};
//!
//! let options = Options::default()
//!     .case_insensitive(true)
//!     .wildcard_any_with(b'%');
//!
//! assert_eq!(
//!     dowild_with("foo%".as_bytes(), "FOOBAR".as_bytes(), options),
//!     true
//! );
//! ```
//!
//! With the [`SimpleMatch`] trait in scope, the [`dowild_with`] function can be accessed
//! directly on the string or u8 slice, ...:
//!
//! ```rust
//! use simplematch::{Options, SimpleMatch};
//!
//! assert_eq!(
//!     "FOObar".dowild_with("foo*", Options::default().case_insensitive(true)),
//!     true
//! );
//! ```
//!
//! ## Character classes
//!
//! An expression `[...]` matches a single character if the first character following the
//! leading `[` is not an `!`. The contents of the brackets must not be empty otherwise the
//! brackets are interpreted literally (the pattern `a[]c` matches `a[]c` exactly); however, a
//! `]` can be included as the first character within the brackets. For example, `[][!]`
//! matches the three characters `[`, `]`, and `!`.
//!
//! ## Ranges
//!
//! A special convention exists where two characters separated by `-` represent a range.
//! For instance, `[A-Fa-f0-9]` is equivalent to `[ABCDEFabcdef0123456789]`.
//! To include `-` as a literal character, it must be placed as the first or last character
//! within the brackets. For example, `[]-]` matches the two characters `]` and `-`. As opposed
//! to regex, it is possible to revert a range `[F-A]` which has the same meaning as `[A-F]`.
//!
//! ## Complementation
//!
//! An expression `[!...]` matches any single character that is not included in the expression
//! formed by removing the first `!`. For example, `[!]a-]` matches any character except `]`,
//! `a`, and `-`.
//!
//! To remove the special meanings of `?`, `*`, and `[`, you can precede them with the escape
//! character (per default the backslash character `\`). Within brackets, these characters
//! represent themselves. For instance, `[[?*\\]` matches the four characters `[`, `?`, `*`,
//! and `\`.
//!
//! ## Credits
//!
//! This linear-time wildcard matching algorithm is derived from the one presented in Russ
//! Cox's great article about simple and performant glob matching (<https://research.swtch.com/glob>).
//! Furthermore, the optimizations for the `?` handling are based on the article [Matching
//! Wildcards: An Improved Algorithm for Big
//! Data](https://developforperformance.com/MatchingWildcards_AnImprovedAlgorithmForBigData.html)
//! written by Kirk J. Krauss.
//!
//! The `simplematch` algorithm is an improved version which uses generally about 2-6x less
//! instructions than the original algorithm for small and big data.

// spell-checker: ignore aaabc fooa Krauss

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// TODO: Remove
#![allow(missing_docs)]

macro_rules! impl_simplematch {
    ( $type:ty: $for:ty ) => {
        impl SimpleMatch<$type> for $for {
            fn dowild(&self, pattern: Self) -> bool {
                dowild(pattern, self)
            }

            fn dowild_with(&self, pattern: Self, options: Options<$type>) -> bool {
                dowild_with(pattern, self, options)
            }
        }
    };
    ( $type:ty: $for:ty => $( $tail:tt )* ) => {
        impl SimpleMatch<$type> for $for {
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
use alloc::collections::VecDeque;
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::cmp::Ordering;
#[cfg(feature = "std")]
use std::collections::VecDeque;
#[cfg(feature = "std")]
use std::string::String;
#[cfg(feature = "std")]
use std::vec::Vec;

pub trait SimpleMatch<T>
where
    T: Wildcard,
{
    #[must_use]
    fn dowild(&self, pattern: Self) -> bool;
    #[must_use]
    fn dowild_with(&self, pattern: Self, options: Options<T>) -> bool;
}

pub trait Wildcard: Eq + Copy + Clone {
    const DEFAULT_ANY: Self;
    const DEFAULT_ESCAPE: Self;
    const DEFAULT_ONE: Self;
    const DEFAULT_RANGE_CLOSE: Self;
    const DEFAULT_RANGE_HYPHEN: Self;
    const DEFAULT_RANGE_NEGATE: Self;
    const DEFAULT_RANGE_OPEN: Self;

    fn match_one(first: &Self, second: &Self, case_sensitive: bool) -> bool;
    fn match_range(token: &Self, low: &Self, high: &Self, case_sensitive: bool) -> bool;
}

#[derive(Debug)]
enum Ranges<T> {
    Positive(Vec<RangeKind<T>>),
    Negative(Vec<RangeKind<T>>),
}

#[derive(Debug)]
enum RangeKind<T> {
    Range(T, T),
    One(T),
    OneRange(T),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Options<T>
where
    T: Wildcard,
{
    pub case_sensitive: bool,
    pub is_ranges_enabled: bool,
    pub range_negate: Option<T>,
    pub wildcard_any: Option<T>,
    pub wildcard_escape: Option<T>,
    pub wildcard_one: Option<T>,
}

#[derive(Debug)]
struct RangePattern<T> {
    end: usize,
    ranges: Option<Ranges<T>>,
    start: usize,
}

#[derive(Debug)]
struct RangePatterns<T>(VecDeque<RangePattern<T>>);

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
            wildcard_escape: None,
            is_ranges_enabled: false,
            range_negate: Some(T::DEFAULT_RANGE_NEGATE),
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
    pub const fn enable_escape(self, yes: bool) -> Self {
        Self {
            wildcard_escape: if yes { Some(T::DEFAULT_ESCAPE) } else { None },
            ..self
        }
    }

    #[must_use]
    pub const fn enable_escape_with(self, token: T) -> Self {
        Self {
            wildcard_escape: Some(token),
            ..self
        }
    }

    #[must_use]
    pub const fn enable_ranges(self, yes: bool) -> Self {
        Self {
            is_ranges_enabled: yes,
            ..self
        }
    }

    #[must_use]
    pub const fn enable_ranges_with(self, negation: T) -> Self {
        Self {
            is_ranges_enabled: true,
            range_negate: Some(negation),
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

impl<T> Ranges<T>
where
    T: Wildcard + Ord,
{
    #[inline]
    fn new_positive(index: usize, length: usize) -> Self {
        Self::Positive(Vec::with_capacity(length - index))
    }

    #[inline]
    fn new_negative(index: usize, length: usize) -> Self {
        Self::Negative(Vec::with_capacity(length - index))
    }

    #[inline]
    fn push(&mut self, kind: RangeKind<T>) {
        match self {
            Self::Positive(range_kinds) | Self::Negative(range_kinds) => range_kinds.push(kind),
        }
    }

    #[inline]
    fn is_match(&self, token: T, case_sensitive: bool) -> bool {
        match self {
            Self::Positive(range_kinds) => range_kinds
                .iter()
                .any(|r| r.contains(&token, case_sensitive)),
            Self::Negative(range_kinds) => !range_kinds
                .iter()
                .any(|r| r.contains(&token, case_sensitive)),
        }
    }
}

impl<T> RangeKind<T>
where
    T: Wildcard + Ord,
{
    #[inline]
    fn contains(&self, token: &T, case_sensitive: bool) -> bool {
        match self {
            Self::Range(low, high) => T::match_range(token, low, high, case_sensitive),
            Self::One(c) | Self::OneRange(c) => T::match_one(c, token, case_sensitive),
        }
    }

    /// Does no out of bounds check for the first character
    #[inline]
    fn parse(index: usize, pattern: &[T]) -> Option<Self> {
        if pattern[index] == T::DEFAULT_RANGE_CLOSE {
            None
        } else {
            Some(Self::parse_first(index, pattern))
        }
    }

    /// Does no out of bounds and `]` check for the first character
    fn parse_first(index: usize, pattern: &[T]) -> Self {
        let first = pattern[index];
        if index + 2 < pattern.len() && pattern[index + 1] == T::DEFAULT_RANGE_HYPHEN {
            let second = pattern[index + 2];
            if second == T::DEFAULT_RANGE_CLOSE {
                Self::One(first)
            } else {
                match first.cmp(&second) {
                    Ordering::Less => Self::Range(first, second),
                    Ordering::Equal => Self::OneRange(first),
                    Ordering::Greater => Self::Range(second, first),
                }
            }
        } else {
            Self::One(first)
        }
    }

    #[inline]
    const fn len(&self) -> usize {
        match self {
            Self::Range(_, _) | Self::OneRange(_) => 3,
            Self::One(_) => 1,
        }
    }
}

impl<T> RangePattern<T>
where
    T: Wildcard + Ord,
{
    #[inline]
    const fn new(brackets: Option<Ranges<T>>, start: usize, end: usize) -> Self {
        Self {
            ranges: brackets,
            start,
            end,
        }
    }

    #[inline]
    const fn new_invalid(start: usize, end: usize) -> Self {
        Self::new(None, start, end)
    }

    #[inline]
    const fn len(&self) -> usize {
        self.end - self.start + 1
    }

    fn parse(start: usize, pattern: &[T], range_negate: T) -> Self {
        // The first character of a range is always the opening bracket
        let mut p_idx = start + 1;
        if p_idx + 2 > pattern.len() {
            // The pattern is too short to produce a valid range
            return Self::new_invalid(start, p_idx + 1);
        }

        let mut ranges = if pattern[p_idx] == range_negate {
            p_idx += 1;
            Ranges::new_negative(p_idx, pattern.len())
        } else {
            Ranges::new_positive(p_idx, pattern.len())
        };

        // The `]` directly after the opening `[` (and possibly `!`) is special and matched literally
        if pattern[p_idx] == T::DEFAULT_RANGE_CLOSE {
            let kind = RangeKind::parse_first(p_idx, pattern);
            p_idx += kind.len();
            ranges.push(kind);
        }

        if p_idx < pattern.len() {
            // Parse until we reach either the end of the string or find a `]`
            while let Some(kind) = RangeKind::parse(p_idx, pattern) {
                p_idx += kind.len();
                if p_idx >= pattern.len() {
                    // The end of the string without a `]`
                    return Self::new_invalid(start, p_idx);
                }
                ranges.push(kind);
            }

            // The `None` case tells us we've found a `]` and a valid range
            Self::new(Some(ranges), start, p_idx)
        } else {
            // We've reached the end of the string without a closing `]`
            Self::new_invalid(start, p_idx)
        }
    }

    #[inline]
    fn try_match(&self, token: T, case_sensitive: bool) -> Option<bool> {
        self.ranges
            .as_ref()
            .map(|ranges| ranges.is_match(token, case_sensitive))
    }
}

impl<T> RangePatterns<T>
where
    T: Wildcard + Ord,
{
    #[inline]
    fn new() -> Self {
        Self(VecDeque::new())
    }

    #[inline]
    fn get(&self, index: usize) -> Option<&RangePattern<T>> {
        self.0.iter().find(|r| r.start == index)
    }

    fn get_or_add(&mut self, start: usize, pattern: &[T], range_negate: T) -> &RangePattern<T> {
        if self.0.capacity() == 0 {
            self.0.reserve(pattern.len() - start);
        }
        if let Some(last) = self.0.back() {
            if last.start >= start {
                return self.get(start).unwrap();
            }
        }

        let pattern = RangePattern::parse(start, pattern, range_negate);
        self.0.push_back(pattern);
        self.0.back().unwrap()
    }

    #[inline]
    fn prune(&mut self, index: usize) {
        while let Some(first) = self.0.front() {
            if first.start < index {
                self.0.pop_front();
            } else {
                break;
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Our trait implementations for the basic types
////////////////////////////////////////////////////////////////////////////////

impl_simplematch!(u8: &[u8]);
impl_simplematch!(u8: &str => .as_bytes());
impl_simplematch!(u8: String => .as_bytes());
impl_simplematch!(u8: Vec<u8> => .as_slice());
impl_simplematch!(char: &[char]);
impl_simplematch!(char: Vec<char> => .as_slice());

impl Wildcard for u8 {
    const DEFAULT_ANY: Self = b'*';
    const DEFAULT_ESCAPE: Self = b'\\';
    const DEFAULT_ONE: Self = b'?';
    const DEFAULT_RANGE_CLOSE: Self = b']';
    const DEFAULT_RANGE_HYPHEN: Self = b'-';
    const DEFAULT_RANGE_NEGATE: Self = b'!';
    const DEFAULT_RANGE_OPEN: Self = b'[';

    #[inline]
    fn match_one(first: &Self, second: &Self, case_sensitive: bool) -> bool {
        if case_sensitive {
            first == second
        } else {
            first.eq_ignore_ascii_case(second)
        }
    }

    #[inline]
    fn match_range(token: &Self, low: &Self, high: &Self, case_sensitive: bool) -> bool {
        if case_sensitive {
            low <= token && token <= high
        } else {
            let token_lowercase = token.to_ascii_lowercase();
            // The token is not alphabetic
            if token_lowercase == *token {
                low <= token && token <= high
            } else {
                low.to_ascii_lowercase() <= token_lowercase
                    && token_lowercase <= high.to_ascii_lowercase()
            }
        }
    }
}

impl Wildcard for char {
    const DEFAULT_ANY: Self = '*';
    const DEFAULT_ESCAPE: Self = '\\';
    const DEFAULT_ONE: Self = '?';
    const DEFAULT_RANGE_CLOSE: Self = ']';
    const DEFAULT_RANGE_HYPHEN: Self = '-';
    const DEFAULT_RANGE_NEGATE: Self = '!';
    const DEFAULT_RANGE_OPEN: Self = '[';

    #[inline]
    fn match_one(first: &Self, second: &Self, case_sensitive: bool) -> bool {
        if case_sensitive {
            first == second
        } else {
            first.to_lowercase().eq(second.to_lowercase())
        }
    }

    #[inline]
    fn match_range(token: &Self, low: &Self, high: &Self, case_sensitive: bool) -> bool {
        if case_sensitive {
            low <= token && token <= high
        } else {
            let token_lowercase = token.to_ascii_lowercase();
            // The token is not ascii alphabetic
            if token_lowercase == *token {
                low <= token && token <= high
            } else {
                low.to_ascii_lowercase() <= token_lowercase
                    && token_lowercase <= high.to_ascii_lowercase()
            }
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
/// This is the basic algorithm without customization options to provide the best performance.
/// If you need more [`Options`] use [`dowild_with`].
///
/// Match directly on strings, u8 slices, ... without performance loss, if you bring the
/// [`SimpleMatch`] trait in scope.
///
/// See also the [library documentation](crate) for more details.
///
/// # Examples
///
/// ```rust
/// use simplematch::dowild;
///
/// assert_eq!(dowild("*bc".as_bytes(), "aaabc".as_bytes()), true);
/// ```
///
/// or more conveniently directly on a string
///
/// ```rust
/// use simplematch::SimpleMatch;
///
/// assert_eq!("aaabc".dowild("*bc"), true);
/// ```
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

    let mut has_seen_wildcard_any = false;
    while p_idx < pattern.len() || h_idx < haystack.len() {
        if p_idx < pattern.len() {
            match pattern[p_idx] {
                // This (expensive) case is ensured to be entered only once per `wildcard_any`
                // character in the pattern.
                c if c == wildcard_any => {
                    has_seen_wildcard_any = true;
                    p_idx += 1;

                    while p_idx < pattern.len() && pattern[p_idx] == wildcard_any {
                        p_idx += 1;
                    }
                    if p_idx >= pattern.len() {
                        return true;
                    }

                    let next_c = pattern[p_idx];
                    if next_c == wildcard_one {
                        // 1. This optimization prevents checking for the same `wildcard_one`
                        //    character in the big loop again.
                        // 2. More importantly for the performance, we can advance the pattern and
                        //    haystack for all index counters including `next_h_idx` and
                        //    `next_p_idx`.
                        while h_idx < haystack.len() {
                            p_idx += 1;
                            h_idx += 1;
                            if !(p_idx < pattern.len() && pattern[p_idx] == next_c) {
                                break;
                            }
                        }
                    } else {
                        // Advancing the haystack and `next_h_idx` counter to the first match
                        // significantly enhances the overall performance.
                        while h_idx < haystack.len() && haystack[h_idx] != next_c {
                            h_idx += 1;
                        }
                        if h_idx >= haystack.len() {
                            return false;
                        }
                    }

                    // Instead of pinning `next_p_idx` to the `wildcard_any` index and entering this
                    // match case in the big loop again after a reset to the `next` indices, it's
                    // more efficient to pin it to the first character after `wildcard_any` (or
                    // after `wildcard_one` if it is the character after `wildcard_any`).
                    next_p_idx = p_idx;
                    next_h_idx = h_idx;
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
        // If `true`, we need to reset
        if has_seen_wildcard_any && next_h_idx < haystack.len() {
            p_idx = next_p_idx;
            next_h_idx += 1;

            // We don't enter the `wildcard_any` match case in the big loop again, so we have to
            // apply this optimization from above here again, if applicable.
            if p_idx < pattern.len() {
                while next_h_idx < haystack.len() && haystack[next_h_idx] != pattern[p_idx] {
                    next_h_idx += 1;
                }
            }

            h_idx = next_h_idx;
            continue;
        }

        return false;
    }

    // The pattern and the haystack are both exhausted which means we have a match
    true
}

/// Return true if the wildcard pattern matches the `haystack`. This method can be customized
/// with [`Options`].
///
/// Don't use this method if you only need the default [`Options`]. The [`dowild`] function is
/// more performant in such cases.
///
/// Like with [`dowild`], allowed wildcard characters are `*` to match any amount of characters
/// and `?` to match exactly one character.
///
/// The [`Options`] structure allows for case-insensitive matching. You can customize the
/// `wildcard_any` character (`*`) and the `wildcard_one` character (`?`). Escaping can also
/// be enabled, allowing you to specify a custom escape character. Additionally, character
/// classes and ranges, such as `[a-z]`, are supported, and the negation character can be
/// customized to match all characters not included in a specified range, as in `[!a-z]`.
///
/// See also the [library documentation](crate) for more details.
///
/// # Examples
///
/// ```rust
/// use simplematch::{dowild_with, Options};
///
/// assert_eq!(
///     dowild_with(
///         "*bc".as_bytes(),
///         "AAabc".as_bytes(),
///         Options::default().case_insensitive(true)
///     ),
///     true
/// );
/// ```
///
/// or more conveniently match directly on a string bringing the [`SimpleMatch`] trait in
/// scope.
///
/// ```rust
/// use simplematch::{Options, SimpleMatch};
///
/// assert_eq!(
///     "aaabc".dowild_with("%bc", Options::default().wildcard_any_with(b'%')),
///     true
/// );
/// ```
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn dowild_with<T>(pattern: &[T], haystack: &[T], options: Options<T>) -> bool
where
    T: Wildcard + Ord,
{
    let Options {
        case_sensitive,
        is_ranges_enabled,
        range_negate,
        wildcard_any,
        wildcard_escape,
        wildcard_one,
    } = options;

    let range_negate = range_negate.unwrap_or(T::DEFAULT_RANGE_NEGATE);
    let wildcard_any = wildcard_any.unwrap_or(T::DEFAULT_ANY);
    let wildcard_one = wildcard_one.unwrap_or(T::DEFAULT_ONE);
    let (is_escape_enabled, wildcard_escape) = match wildcard_escape {
        Some(x) => (true, x),
        // although the value for `escape` is not used we need to assign some reasonable value
        None => (false, T::DEFAULT_ESCAPE),
    };

    let mut p_idx = 0;
    let mut h_idx = 0;

    let mut next_p_idx = 0;
    let mut next_h_idx = 0;

    let mut ranges = RangePatterns::new();

    let mut has_seen_wildcard_any = false;
    while p_idx < pattern.len() || h_idx < haystack.len() {
        if p_idx < pattern.len() {
            match pattern[p_idx] {
                c if c == wildcard_any => {
                    has_seen_wildcard_any = true;
                    p_idx += 1;

                    while p_idx < pattern.len() && pattern[p_idx] == wildcard_any {
                        p_idx += 1;
                    }
                    if p_idx >= pattern.len() {
                        return true;
                    }

                    let next_c = pattern[p_idx];
                    #[allow(clippy::else_if_without_else)]
                    if next_c == wildcard_one {
                        while h_idx < haystack.len() {
                            p_idx += 1;
                            h_idx += 1;
                            if !(p_idx < pattern.len() && pattern[p_idx] == next_c) {
                                break;
                            }
                        }
                    } else if !((is_escape_enabled && next_c == wildcard_escape)
                        || (is_ranges_enabled && next_c == T::DEFAULT_RANGE_OPEN))
                    {
                        while h_idx < haystack.len()
                            && !T::match_one(&haystack[h_idx], &next_c, case_sensitive)
                        {
                            h_idx += 1;
                        }
                        if h_idx >= haystack.len() {
                            return false;
                        }
                    }

                    next_p_idx = p_idx;
                    next_h_idx = h_idx;
                    continue;
                }
                c if c == wildcard_one => {
                    if h_idx < haystack.len() {
                        p_idx += 1;
                        h_idx += 1;
                        continue;
                    }
                }
                c if is_escape_enabled && c == wildcard_escape && p_idx + 1 < pattern.len() => {
                    if h_idx < haystack.len() {
                        let next_c = pattern[p_idx + 1];
                        let h = haystack[h_idx];

                        let is_special = next_c == wildcard_any
                            || next_c == wildcard_one
                            || next_c == wildcard_escape
                            || (is_ranges_enabled && next_c == T::DEFAULT_RANGE_OPEN);
                        #[allow(clippy::else_if_without_else)]
                        if is_special && h == next_c {
                            p_idx += 2;
                            h_idx += 1;
                            continue;
                        } else if !is_special && h == wildcard_escape {
                            p_idx += 1;
                            h_idx += 1;
                            continue;
                        }
                    }
                }
                c if is_ranges_enabled
                    && c == T::DEFAULT_RANGE_OPEN
                    && p_idx + 1 < pattern.len() =>
                {
                    if h_idx < haystack.len() {
                        if has_seen_wildcard_any {
                            ranges.prune(next_p_idx);
                        }

                        let range = ranges.get_or_add(p_idx, pattern, range_negate);
                        #[allow(clippy::else_if_without_else)]
                        if let Some(is_match) = range.try_match(haystack[h_idx], case_sensitive) {
                            p_idx += range.len();
                            if is_match {
                                h_idx += 1;
                                continue;
                            }
                        } else if T::match_one(
                            &haystack[h_idx],
                            &T::DEFAULT_RANGE_OPEN,
                            case_sensitive,
                        ) {
                            p_idx += 1;
                            h_idx += 1;
                            continue;
                        }
                    }
                }
                c => {
                    if h_idx < haystack.len() && T::match_one(&haystack[h_idx], &c, case_sensitive) {
                        p_idx += 1;
                        h_idx += 1;
                        continue;
                    }
                }
            }
        }
        if has_seen_wildcard_any && next_h_idx < haystack.len() {
            p_idx = next_p_idx;
            next_h_idx += 1;

            if p_idx < pattern.len()
                && !(is_ranges_enabled && pattern[p_idx] == T::DEFAULT_RANGE_OPEN)
                && !(is_escape_enabled && pattern[p_idx] == wildcard_escape)
            {
                while next_h_idx < haystack.len()
                    && !T::match_one(&haystack[next_h_idx], &pattern[p_idx], case_sensitive)
                {
                    next_h_idx += 1;
                }
            }

            h_idx = next_h_idx;
            continue;
        }

        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::same_case_sensitive(b'j', b'j', true, true)]
    #[case::different_case_case_sensitive(b'j', b'J', true, false)]
    #[case::different_char_case_sensitive(b'a', b'b', true, false)]
    #[case::same_case_insensitive(b'j', b'j', false, true)]
    #[case::different_case_insensitive(b'j', b'J', false, true)]
    #[case::different_char_case_insensitive(b'a', b'B', false, false)]
    fn impl_wildcard_match_one(
        #[case] first: u8,
        #[case] second: u8,
        #[case] case_sensitive: bool,
        #[case] expected: bool,
    ) {
        assert_eq!(
            Wildcard::match_one(&first, &second, case_sensitive),
            expected
        );
        assert_eq!(
            Wildcard::match_one(&(first as char), &(second as char), case_sensitive),
            expected
        );
    }

    #[rstest]
    #[case::all_the_same(b'j', b'j', b'j', true)]
    #[case::low_is_higher_high_is_same(b'j', b'k', b'j', false)]
    #[case::low_is_lower_high_is_same(b'j', b'i', b'j', true)]
    #[case::high_is_lower_low_is_same(b'j', b'k', b'i', false)]
    #[case::high_is_higher_low_is_same(b'j', b'j', b'k', true)]
    fn impl_wildcard_match_range_when_case_sensitive(
        #[case] token: u8,
        #[case] low: u8,
        #[case] high: u8,
        #[case] expected: bool,
    ) {
        assert_eq!(Wildcard::match_range(&token, &low, &high, true), expected);
        assert_eq!(
            Wildcard::match_range(&(token as char), &(low as char), &(high as char), true),
            expected
        );
    }

    #[rstest]
    #[case::all_the_same_small(b'j', b'j', b'j', true)]
    #[case::all_the_same_big(b'J', b'J', b'J', true)]
    // This token is one of the characters between `Z` and `a`
    #[case::no_alpha_low_is_big(b'[', b'A', b'z', true)]
    #[case::no_alpha_both_big(b'[', b'A', b'Z', false)]
    #[case::no_alpha_low_is_small(b'[', b'a', b'z', false)]
    #[case::no_alpha_both_small(b'[', b'a', b'z', false)]
    #[case::all_small_middle(b'j', b'a', b'z', true)]
    #[case::all_small_low_is_higher(b'j', b'k', b'z', false)]
    #[case::all_small_high_is_lower(b'j', b'a', b'i', false)]
    #[case::all_big_middle(b'J', b'A', b'Z', true)]
    #[case::all_big_low_is_higher(b'J', b'K', b'Z', false)]
    #[case::all_big_high_is_lower(b'J', b'A', b'I', false)]
    #[case::fuzz(b']', b'/', b'A', false)]
    fn impl_wildcard_match_range_when_case_insensitive(
        #[case] token: u8,
        #[case] low: u8,
        #[case] high: u8,
        #[case] expected: bool,
    ) {
        assert_eq!(Wildcard::match_range(&token, &low, &high, false), expected);
        assert_eq!(
            Wildcard::match_range(&(token as char), &(low as char), &(high as char), false),
            expected
        );
    }
}
