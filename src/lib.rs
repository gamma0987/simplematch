//! The library

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

// TODO: CLEANUP if not used anywhere
pub const DEFAULT_ESCAPE: u8 = b'\\';
pub const DEFAULT_WILDCARD_ANY: u8 = b'*';
pub const DEFAULT_WILDCARD_ONE: u8 = b'?';

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Options<T>
where
    T: Wildcard,
{
    pub case_sensitive: bool,
    pub escape: Option<T>,
    pub is_ranges_enabled: bool,
    pub range_negate: Option<T>,
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

    // TODO: Range with customizable negation char
    #[must_use]
    pub const fn enable_ranges(self) -> Self {
        Self {
            is_ranges_enabled: true,
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
            low.to_ascii_lowercase() <= token_lowercase
                && token_lowercase <= high.to_ascii_lowercase()
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
            let token_lowercase = token.to_lowercase();
            match low.to_lowercase().cmp(token_lowercase.clone()) {
                Ordering::Less | Ordering::Equal => match token_lowercase.cmp(high.to_lowercase()) {
                    Ordering::Less | Ordering::Equal => true,
                    Ordering::Greater => false,
                },
                Ordering::Greater => false,
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

/// TODO: SORT ALL, derive
#[derive(Debug)]
enum RangeKind<T> {
    Range(T, T),
    One(T),
}

impl<T> RangeKind<T>
where
    T: Wildcard + Ord,
{
    #[inline]
    fn contains(&self, token: &T, case_sensitive: bool) -> bool {
        match self {
            Self::Range(low, high) => T::match_range(token, low, high, case_sensitive),
            Self::One(c) => T::match_one(c, token, case_sensitive),
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
                    Ordering::Equal => Self::One(first),
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
            Self::Range(_, _) => 3,
            Self::One(_) => 1,
        }
    }
}

#[derive(Debug)]
enum Ranges<T> {
    Positive(Vec<RangeKind<T>>),
    Negative(Vec<RangeKind<T>>),
}

impl<T> Ranges<T>
where
    T: Wildcard + Ord,
{
    fn new_positive(kind: Option<RangeKind<T>>, index: usize, length: usize) -> Self {
        let mut this = Self::Positive(Vec::with_capacity(length - index));
        if let Some(kind) = kind {
            this.push(kind);
        }
        this
    }

    // TODO: refactor with `new_positive`
    fn new_negative(kind: Option<RangeKind<T>>, index: usize, length: usize) -> Self {
        let mut this = Self::Negative(Vec::with_capacity(length - index));
        if let Some(kind) = kind {
            this.push(kind);
        }
        this
    }

    #[inline]
    fn push(&mut self, kind: RangeKind<T>) {
        match self {
            Self::Positive(range_kinds) | Self::Negative(range_kinds) => range_kinds.push(kind),
        }
    }

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

#[derive(Debug)]
struct RangePattern<T> {
    end: usize,
    ranges: Option<Ranges<T>>,
    start: usize,
}

impl<T> RangePattern<T>
where
    T: Wildcard + Ord,
{
    #[inline]
    const fn len(&self) -> usize {
        self.end - self.start + 1
    }

    /// If An expression `[...]` where the first character after the leading `[` is not an `!`
    /// matches a single character, namely any of the characters enclosed by the brackets.  The
    /// string enclosed by the brackets cannot be empty; therefore `]` can be allowed between
    /// the brackets, provided that it is the first character.
    // (Thus, `[][!]` matches the three characters `[`, `]`, and `!`.)
    //
    // # Ranges
    //
    // There  is  one special convention: two characters separated by `-` denote a range.  (Thus,
    // `[A-Fa-f0-9]` is equivalent to `[ABCDEFabcdef0123456789]`.)  One may include `-` in its
    // literal meaning by making it the first or last character between the brackets.  (Thus,
    // `[]-]` matches just the two characters `]` and `-`, and `[--0]` matches the three
    // characters `-`, `.`, and `0`, since `/` cannot be matched.)
    //
    // # Complementation
    //
    // An expression `[!...]` matches a single character, namely any character that is not matched
    // by the expression obtained by removing the first `!` from it.  (Thus, `[!]a-]` matches any
    // single character except `]`, `a`, and `-`.)
    //
    // One can remove the special meaning of `?`, `*`, and `[` by preceding them by a backslash,
    // or, in case this is part of a shell command line, enclosing them in quotes.  Between
    // brackets these characters stand for themselves.  Thus, `[[?*\]` matches the four characters
    // `[`, `?`, `*`, and `\`.
    fn parse(start: usize, pattern: &[T], range_negate: T) -> Self {
        // The first character is always the opening bracket
        let mut p_idx = start + 1;
        if p_idx + 2 > pattern.len() {
            return Self::new_invalid(start, p_idx + 1);
        }

        let c = pattern[p_idx];
        let mut ranges = if c == range_negate {
            p_idx += 1;
            Ranges::new_negative(None, p_idx, pattern.len())
        } else {
            Ranges::new_positive(None, p_idx, pattern.len())
        };

        if c == T::DEFAULT_RANGE_CLOSE {
            let kind = RangeKind::parse_first(p_idx, pattern);
            p_idx += kind.len();
            ranges.push(kind);
        }

        while let Some(kind) = RangeKind::parse(p_idx, pattern) {
            p_idx += kind.len();
            if p_idx >= pattern.len() {
                return Self::new_invalid(start, p_idx);
            }
            ranges.push(kind);
        }

        Self::new(Some(ranges), start, p_idx)
    }

    #[inline]
    fn try_match(&self, token: T, case_sensitive: bool) -> Option<bool> {
        self.ranges
            .as_ref()
            .map(|ranges| ranges.is_match(token, case_sensitive))
    }

    #[inline]
    const fn new_invalid(start: usize, end: usize) -> Self {
        Self::new(None, start, end)
    }

    #[inline]
    const fn new(brackets: Option<Ranges<T>>, start: usize, end: usize) -> Self {
        Self {
            ranges: brackets,
            start,
            end,
        }
    }
}

#[derive(Debug)]
struct RangePatterns<T>(VecDeque<RangePattern<T>>);

// TODO: Try with PartialOrd
impl<T> RangePatterns<T>
where
    T: Wildcard + Ord,
{
    #[inline]
    fn new() -> Self {
        Self(VecDeque::new())
    }

    #[inline]
    fn get_or_add(&mut self, start: usize, pattern: &[T], range_negate: T) -> &RangePattern<T> {
        if self.0.capacity() == 0 {
            self.0.reserve(pattern.len() - start);
        }
        if let Some(last) = self.0.back() {
            if last.start >= start {
                // TODO: or use rfind?
                return self.0.iter().find(|r| r.start == start).unwrap();
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

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn dowild_with<T>(pattern: &[T], haystack: &[T], options: Options<T>) -> bool
where
    T: Wildcard + Ord,
{
    let Options {
        case_sensitive,
        escape,
        wildcard_any,
        wildcard_one,
        is_ranges_enabled,
        range_negate,
    } = options;

    let range_negate = range_negate.unwrap_or(T::DEFAULT_RANGE_NEGATE);
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

    let mut ranges = RangePatterns::new();

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
                c if is_ranges_enabled
                    && c == T::DEFAULT_RANGE_OPEN
                    && p_idx + 1 < pattern.len() =>
                {
                    if h_idx < haystack.len() {
                        if 0 < next_p_idx {
                            ranges.prune(next_p_idx);
                        }

                        let range = ranges.get_or_add(p_idx, pattern, range_negate);
                        if let Some(is_match) = range.try_match(haystack[h_idx], case_sensitive) {
                            p_idx += range.len();
                            if is_match {
                                h_idx += 1;
                                continue;
                            }
                        } else {
                            if h_idx < haystack.len()
                                && T::match_one(&haystack[h_idx], &c, case_sensitive)
                            {
                                p_idx += 1;
                                h_idx += 1;
                            }
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
        if 0 < next_h_idx && next_h_idx <= haystack.len() {
            p_idx = next_p_idx;
            h_idx = next_h_idx;
            continue;
        }
        return false;
    }
    true
}
