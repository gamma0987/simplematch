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
//! simplematch = "0.3.0"
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
//! Matches the given `haystack` against the specified `pattern` using simple wildcard rules.
//! The `*` character matches any sequence of characters, while the `?` character matches
//! a single character.
//!
//! `Wildcard` is natively implemented for `u8` and `char`.
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
//! Or, bringing the trait [`DoWild`] in scope allows for more convenient access to this
//! function without performance loss:
//!
//! ```rust
//! use simplematch::DoWild;
//!
//! assert_eq!("foo*".dowild("foobar"), true);
//! ```
//!
//! A possible usage with `char`:
//!
//! ```rust
//! use simplematch::DoWild;
//!
//! let pattern = "foo*".chars().collect::<Vec<char>>();
//! let haystack = "foobar".chars().collect::<Vec<char>>();
//!
//! assert_eq!(pattern.dowild(haystack), true);
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
//! Matches the given `haystack` against the specified `pattern` with customizable [`Options`].
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
//! Like [`dowild`], the [`dowild_with`] function can be accessed directly on the string or u8
//! slice, ...:
//!
//! ```rust
//! use simplematch::{DoWild, Options};
//!
//! assert_eq!(
//!     "foo*".dowild_with("FOObar", Options::default().case_insensitive(true)),
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
//! instructions than the original algorithm; tested with random small and big data.

// spell-checker: ignore aaabc fooa Krauss

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

macro_rules! impl_dowild {
    ( $type:ty: $for:ty ) => {
        impl DoWild<$type> for $for {
            fn dowild(&self, haystack: Self) -> bool {
                dowild(self, haystack)
            }

            fn dowild_with(&self, haystack: Self, options: Options<$type>) -> bool {
                dowild_with(self, haystack, options)
            }
        }
    };
    ( $type:ty: $for:ty => $( $tail:tt )* ) => {
        impl DoWild<$type> for $for {
            fn dowild(&self, haystack: Self) -> bool {
                dowild(self $( $tail )*, haystack $( $tail )* )
            }

            fn dowild_with(&self, haystack: Self, options: Options<$type>) -> bool {
                dowild_with(self $( $tail )*, haystack $( $tail )*, options)
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
use core::fmt::Display;
use core::ops::Deref;
#[cfg(feature = "std")]
use std::collections::VecDeque;
#[cfg(feature = "std")]
use std::error::Error;
#[cfg(feature = "std")]
use std::string::String;
#[cfg(feature = "std")]
use std::vec::Vec;

/// A convenience trait to use [`dowild`] and [`dowild_with`] directly for this type
///
/// This trait is natively implemented for
///
/// * `&str`
/// * `String`
/// * `&[u8]`
/// * `Vec<u8>`
/// * `&[char]`
/// * `Vec<char>`
///
/// # Examples
///
/// Use [`dowild`] directly on a `&str`
///
/// ```rust
/// use simplematch::DoWild;
///
/// assert_eq!("foo*".dowild("foobar"), true);
/// ```
pub trait DoWild<T>
where
    T: Wildcard,
{
    /// Matches this `pattern` against the specified `haystack` using simple wildcard rules.
    ///
    /// See [`dowild`] for more details.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simplematch::DoWild;
    ///
    /// assert_eq!("foo*".dowild("foobar"), true);
    /// ```
    #[must_use]
    fn dowild(&self, haystack: Self) -> bool;

    /// Matches this `pattern` against the specified `haystack` with customizable [`Options`].
    ///
    /// See [`dowild_with`] for more details.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simplematch::{DoWild, Options};
    ///
    /// assert_eq!(
    ///     "foo*".dowild_with("foobar", Options::default().case_insensitive(true)),
    ///     true
    /// );
    /// ```
    #[must_use]
    fn dowild_with(&self, haystack: Self, options: Options<T>) -> bool;
}

/// The trait for types which should be able to be matched for a wildcard pattern
pub trait Wildcard: Eq + Copy + Clone {
    /// The default token to match any number of characters, usually `*`.
    const DEFAULT_ANY: Self;
    /// The default token to close a character class pattern, usually `]`.
    const DEFAULT_CLASS_CLOSE: Self;
    /// The default token to specify a range, usually `-`.
    const DEFAULT_CLASS_HYPHEN: Self;
    /// The default token to negate a character class, usually `!`.
    const DEFAULT_CLASS_NEGATE: Self;
    /// The default token to open a character class pattern, usually `[`.
    const DEFAULT_CLASS_OPEN: Self;
    /// The default token to escape special characters, usually `\`.
    const DEFAULT_ESCAPE: Self;
    /// The default token match exactly one character, usually `?`.
    const DEFAULT_ONE: Self;

    /// Returns `true` if two character match case-insensitive
    fn match_one_case_insensitive(first: Self, second: Self) -> bool;
    /// Returns `true` if two character match case-sensitive
    fn match_one_case_sensitive(first: Self, second: Self) -> bool;

    /// Returns `true` if the `token` matches the range from `low` to `high` case-insensitive
    fn match_range_case_insensitive(token: Self, low: Self, high: Self) -> bool;
    /// Returns `true` if the `token` matches the range from `low` to `high` case-sensitive
    fn match_range_case_sensitive(token: Self, low: Self, high: Self) -> bool;
}

/// A simple type to hold the borrowed or owned value `T`
///
/// `Cow` would have been an alternative but it requires `std` and we don't need the actual
/// copy-on-write property just a container for borrowed or owned data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum BorrowedOrOwned<'a, T> {
    Borrowed(&'a T),
    Owned(T),
}

#[derive(Debug, Clone)]
enum Class<T> {
    Positive(Vec<ClassKind<T>>),
    Negative(Vec<ClassKind<T>>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ClassKind<T> {
    /// A range like `a-z`
    Range(T, T),
    /// A single character
    One(T),
    /// A range which has the same start and end character like `z-z`
    RangeOne(T),
}

/// The `Error` of the simplematch crate
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SimpleMatchError {
    /// A character in [`Options`] was assigned multiple times
    DuplicateCharacterAssignment,
}

// Represents a character class
#[derive(Debug, Clone)]
struct CharacterClass<T> {
    /// If `None`, the character class is invalid.
    class: Option<Class<T>>,
    /// The end index in the pattern
    end: usize,
    /// The start index in the pattern
    start: usize,
}

#[derive(Debug, Clone)]
struct CharacterClasses<T>(VecDeque<CharacterClass<T>>);

/// Customize the matching behavior of the [`dowild_with`] function
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct Options<T>
where
    T: Wildcard,
{
    /// If `true` the patterns are matched case-sensitive
    ///
    /// The default is to match case-sensitive. Currently, only ascii characters are
    /// considered.
    pub case_sensitive: bool,

    /// The token to negate a character class.
    ///
    /// The default is `!`
    pub class_negate: T,

    /// Set to `true` to enable character classes `[...]`.
    ///
    /// The default is `false`.
    pub is_classes_enabled: bool,

    /// Set to `true` to enable escaping special characters in the pattern.
    ///
    /// The default is `false`.
    ///
    /// The default wildcard characters that can be escaped per default are `*`, `?`. These
    /// characters are adjustable. If character classes are enabled, `[` can be escaped, too.
    ///
    /// If the escape character is not escaping a special character it is matched literally.
    /// For example `"\\a"` matches the escape character and `a` literally.
    pub is_escape_enabled: bool,

    /// The token in the pattern to match zero or more occurrences of any character.
    ///
    /// The default token is `*`.
    pub wildcard_any: T,

    /// The token in the pattern to escape special characters as defined by the other fields.
    ///
    /// The default is the backslash character `\`.
    pub wildcard_escape: T,
    /// The token in the pattern to match exactly one occurrence of any character.
    ///
    /// The default token is `?`.
    pub wildcard_one: T,
}

impl<T> Deref for BorrowedOrOwned<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            BorrowedOrOwned::Borrowed(value) => value,
            BorrowedOrOwned::Owned(value) => value,
        }
    }
}

impl<T> AsRef<T> for BorrowedOrOwned<'_, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T> CharacterClass<T>
where
    T: Wildcard + Ord,
{
    /// Create a new valid character class
    #[inline]
    const fn new(class: Option<Class<T>>, start: usize, end: usize) -> Self {
        Self { class, end, start }
    }

    /// Create a new invalid character class
    #[inline]
    const fn new_invalid(start: usize, end: usize) -> Self {
        Self::new(None, start, end)
    }

    /// Returns the length of this character class.
    #[inline]
    const fn len(&self) -> usize {
        self.end - self.start + 1
    }

    /// Parse a `CharacterClass`  with the opening bracket at the `start` index
    ///
    /// Beware, the starting condition is not verified in any way. A [`CharacterClass`] is
    /// considered invalid, if there is no closing bracket found.
    fn parse(start: usize, pattern: &[T], class_negate: T) -> Self {
        // The first character of a range is always the opening bracket
        let mut p_idx = start + 1;
        if p_idx + 2 > pattern.len() {
            // The pattern is too short to produce a valid range
            return Self::new_invalid(start, p_idx + 1);
        }

        let mut class = if pattern[p_idx] == class_negate {
            p_idx += 1;
            Class::new_negative()
        } else {
            Class::new_positive()
        };

        // The `]` directly after the opening `[` (and possibly `!`) is special and matched literally
        if pattern[p_idx] == T::DEFAULT_CLASS_CLOSE {
            let kind = ClassKind::parse_first(p_idx, pattern);
            p_idx += kind.len();
            class.push(kind);
        }

        if p_idx < pattern.len() {
            // Parse until we reach either the end of the string or find a `]`
            while let Some(kind) = ClassKind::parse(p_idx, pattern) {
                p_idx += kind.len();
                if p_idx >= pattern.len() {
                    // The end of the string without a `]`
                    return Self::new_invalid(start, p_idx);
                }
                class.push(kind);
            }

            // The `None` case tells us we've found a `]` and a valid range
            Self::new(Some(class), start, p_idx)
        } else {
            // We've reached the end of the string without a closing `]`
            Self::new_invalid(start, p_idx)
        }
    }

    /// If this `class` is valid, returns the result of [`Class::is_match`], otherwise `None`
    #[inline]
    fn try_match<F, G>(&self, token: T, match_one: F, match_range: G) -> Option<bool>
    where
        F: Fn(T, T) -> bool + Copy,
        G: Fn(T, T, T) -> bool + Copy,
    {
        self.class
            .as_ref()
            .map(|class| class.is_match(token, match_one, match_range))
    }
}

impl<T> CharacterClasses<T>
where
    T: Wildcard + Ord,
{
    /// Create a new `CharacterClass`
    ///
    /// This method does not allocate any memory.
    #[inline]
    fn new() -> Self {
        Self(VecDeque::new())
    }

    /// Returns the `CharacterClass` with the given `index` as `start` index
    #[inline]
    fn get(&self, index: usize) -> Option<&CharacterClass<T>> {
        self.0.iter().find(|r| r.start == index)
    }

    #[inline]
    fn parse(start: usize, pattern: &[T], class_negate: T) -> CharacterClass<T> {
        CharacterClass::parse(start, pattern, class_negate)
    }

    /// Parse a new class at this `index` or if already present return a reference to it.
    ///
    /// The character at the `index` has to be the opening bracket character. This implies that
    /// `start < pattern.len()`. Note a [`CharacterClass`] can be invalid if there was no
    /// closing bracket.
    fn get_or_add(&mut self, start: usize, pattern: &[T], class_negate: T) -> &CharacterClass<T> {
        if let Some(last) = self.0.back() {
            #[allow(clippy::else_if_without_else)]
            if last.start == start {
                // SAFETY: The equivalent safe code is `return self.0.back().unwrap()`, but calling
                // `back()` again and unwrap is unnecessary in this case. The reference `last` is
                // guaranteed to be valid as it is just obtained from `self.0.back()`. The mutable
                // reference to `self` prevents any concurrent modifications to `self.0` while this
                // function is executing, ensuring that the data remains valid between the call to
                // `back()` and the return here.
                return unsafe { &*(last as *const CharacterClass<T>) };
            // We already parsed this character class
            } else if last.start > start {
                return self.get(start).unwrap();
            }
        }

        let class = Self::parse(start, pattern, class_negate);

        // Stick to the default allocation strategy, doubling the buffer starting with a capacity of
        // `1`. In case of an invalid class as first class, the maximum amount of classes is `1`, so
        // `1` might be a good starting point in any case. The maximum amount of `(pattern.len() -
        // start) / 3` valid classes is most likely too much in typical scenarios.
        self.0.push_back(class);

        // SAFETY: This unwrap is safe since we just added a class
        unsafe { self.0.back().unwrap_unchecked() }
    }

    /// Remove classes that have a smaller starting index than the given `index`
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

impl<T> Class<T>
where
    T: Wildcard + Ord,
{
    /// Create a new positive `Class`.
    #[inline]
    const fn new_positive() -> Self {
        Self::Positive(Vec::new())
    }

    /// Create a new negative `Class`.
    #[inline]
    const fn new_negative() -> Self {
        Self::Negative(Vec::new())
    }

    /// Add a new [`ClassKind`] to this `Class`.
    #[inline]
    fn push(&mut self, kind: ClassKind<T>) {
        match self {
            Self::Positive(kinds) | Self::Negative(kinds) => {
                if kinds.last() != Some(&kind) {
                    kinds.push(kind);
                }
            }
        }
    }

    /// Returns `true` if a positive `Class` contains the given `token` or if negative doesn't
    /// contain the `token`.
    #[inline]
    fn is_match<F, G>(&self, token: T, match_one: F, match_range: G) -> bool
    where
        F: Fn(T, T) -> bool + Copy,
        G: Fn(T, T, T) -> bool + Copy,
    {
        match self {
            Self::Positive(kinds) => kinds
                .iter()
                .any(|r| r.contains(&token, match_one, match_range)),
            Self::Negative(kinds) => !kinds
                .iter()
                .any(|r| r.contains(&token, match_one, match_range)),
        }
    }
}

impl<T> ClassKind<T>
where
    T: Wildcard + Ord,
{
    #[inline]
    fn contains<F, G>(&self, token: &T, match_one: F, match_range: G) -> bool
    where
        F: Fn(T, T) -> bool,
        G: Fn(T, T, T) -> bool,
    {
        match self {
            Self::Range(low, high) => match_range(*token, *low, *high),
            Self::One(c) | Self::RangeOne(c) => match_one(*c, *token),
        }
    }

    /// Does no out of bounds check for the first character
    #[inline]
    fn parse(index: usize, pattern: &[T]) -> Option<Self> {
        if pattern[index] == T::DEFAULT_CLASS_CLOSE {
            None
        } else {
            Some(Self::parse_first(index, pattern))
        }
    }

    /// Does no out of bounds and `]` check for the first character
    fn parse_first(index: usize, pattern: &[T]) -> Self {
        let first = pattern[index];
        if index + 2 < pattern.len() && pattern[index + 1] == T::DEFAULT_CLASS_HYPHEN {
            let second = pattern[index + 2];
            if second == T::DEFAULT_CLASS_CLOSE {
                Self::One(first)
            } else {
                match first.cmp(&second) {
                    Ordering::Less => Self::Range(first, second),
                    Ordering::Equal => Self::RangeOne(first),
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
            Self::Range(_, _) | Self::RangeOne(_) => 3,
            Self::One(_) => 1,
        }
    }
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
    /// Create new `Options` for the [`dowild_with`] function.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            case_sensitive: true,
            wildcard_escape: T::DEFAULT_ESCAPE,
            is_classes_enabled: false,
            class_negate: T::DEFAULT_CLASS_NEGATE,
            wildcard_any: T::DEFAULT_ANY,
            wildcard_one: T::DEFAULT_ONE,
            is_escape_enabled: false,
        }
    }

    /// If `true` match the pattern case-insensitive.
    ///
    /// The default is to match case-sensitive.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simplematch::Options;
    ///
    /// let options: Options<u8> = Options::default().case_insensitive(true);
    /// ```
    #[must_use]
    pub const fn case_insensitive(mut self, yes: bool) -> Self {
        self.case_sensitive = !yes;
        self
    }

    /// If `true` enable escaping of special characters in the pattern.
    ///
    /// The default is `false` and the default escape character is backslash `\`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simplematch::Options;
    ///
    /// let options: Options<u8> = Options::default().enable_escape(true);
    /// ```
    #[must_use]
    pub const fn enable_escape(mut self, yes: bool) -> Self {
        self.is_escape_enabled = yes;
        self
    }

    /// Enable escaping of special characters but use this `token` instead of the default.
    ///
    /// The default is `false` and the default escape character is backslash `\`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simplematch::Options;
    ///
    /// let options = Options::default().enable_escape_with(b'#');
    /// ```
    #[must_use]
    pub const fn enable_escape_with(mut self, token: T) -> Self {
        self.is_escape_enabled = true;
        self.wildcard_escape = token;
        self
    }

    /// If `true`, enable character classes `[...]`.
    ///
    /// The default is `false`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simplematch::Options;
    ///
    /// let options: Options<u8> = Options::default().enable_classes(true);
    /// ```
    #[must_use]
    pub const fn enable_classes(mut self, yes: bool) -> Self {
        self.is_classes_enabled = yes;
        self
    }

    /// If `true`, enable character classes `[...]` but use this `token` for the negation.
    ///
    /// The default is `false` and the default negation character is exclamation mark `!`.
    ///
    /// # Examples
    ///
    /// Set the negation character to the same character as regex uses it.
    ///
    /// ```rust
    /// use simplematch::Options;
    ///
    /// let options = Options::default().enable_classes_with(b'^');
    /// ```
    #[must_use]
    pub const fn enable_classes_with(mut self, negation: T) -> Self {
        self.is_classes_enabled = true;
        self.class_negate = negation;
        self
    }

    /// Use this `token` instead of the default `*` to match any occurrences of a characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simplematch::Options;
    ///
    /// let options = Options::default().wildcard_any_with(b'%');
    /// ```
    #[must_use]
    pub const fn wildcard_any_with(mut self, token: T) -> Self {
        self.wildcard_any = token;
        self
    }

    /// Use this `token` instead of the default `?` to match exactly one character.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simplematch::Options;
    ///
    /// let options = Options::default().wildcard_any_with(b'_');
    #[must_use]
    pub const fn wildcard_one_with(mut self, token: T) -> Self {
        self.wildcard_one = token;
        self
    }

    /// Check `Options` for configuration errors
    ///
    /// An invalid configuration consists of duplicate character assignments. For example you
    /// can't use `*` for the escape character and `wildcard_any` character simultaneously.
    ///
    /// # Errors
    ///
    /// Returns an error if these `Options` are invalid.
    ///
    /// # Examples
    ///
    /// Assigning `?` with [`wildcard_any_with`](Options::wildcard_any_with) fails this method.
    ///
    /// ```rust
    /// use simplematch::{Options, SimpleMatchError};
    ///
    /// assert_eq!(
    ///     Options::default().wildcard_any_with(b'?').verified(),
    ///     Err(SimpleMatchError::DuplicateCharacterAssignment)
    /// );
    /// ```
    pub fn verify(&self) -> Result<(), SimpleMatchError> {
        if self.wildcard_any == self.wildcard_one
            || self.wildcard_any == self.wildcard_escape
            || self.wildcard_any == self.class_negate
            || self.wildcard_one == self.wildcard_escape
            || self.wildcard_one == self.class_negate
            || self.wildcard_escape == self.class_negate
        {
            return Err(SimpleMatchError::DuplicateCharacterAssignment);
        }

        Ok(())
    }

    /// A convenience method that consumes and returns these `Options` if it succeeds.
    ///
    /// The only difference to [`verify`] is, that this method consumes the [`Options`]
    /// returning it on success.
    ///
    /// # Errors
    ///
    /// Returns an error if `Options` are invalid.
    ///
    /// # Examples
    ///
    /// If the configuration is valid, this method returns these `Options`.
    ///
    /// ```rust
    /// use simplematch::{Options, SimpleMatchError};
    ///
    /// let options = Options::default()
    ///     .wildcard_any_with(b'%')
    ///     .verified()
    ///     .unwrap();
    /// ```
    ///
    /// Otherwise, for example assigning `?` with
    /// [`wildcard_any_with`](Options::wildcard_any_with) fails.
    ///
    /// ```rust
    /// use simplematch::{Options, SimpleMatchError};
    ///
    /// assert_eq!(
    ///     Options::default().wildcard_any_with(b'?').verified(),
    ///     Err(SimpleMatchError::DuplicateCharacterAssignment)
    /// );
    /// ```
    ///
    /// [`verify`]: Options::verify
    pub fn verified(self) -> Result<Self, SimpleMatchError> {
        self.verify().map(|()| self)
    }
}

#[cfg(feature = "std")]
impl Error for SimpleMatchError {}

impl Display for SimpleMatchError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DuplicateCharacterAssignment => {
                write!(
                    f,
                    "Verifying options failed: The options contain a duplicate character \
                     assignment."
                )
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Our trait implementations for the basic types
////////////////////////////////////////////////////////////////////////////////

impl_dowild!(u8: &[u8]);
impl_dowild!(u8: &str => .as_bytes());
impl_dowild!(u8: String => .as_bytes());
impl_dowild!(u8: Vec<u8> => .as_slice());
impl_dowild!(char: &[char]);
impl_dowild!(char: Vec<char> => .as_slice());

impl Wildcard for u8 {
    const DEFAULT_ANY: Self = b'*';
    const DEFAULT_ESCAPE: Self = b'\\';
    const DEFAULT_ONE: Self = b'?';
    const DEFAULT_CLASS_CLOSE: Self = b']';
    const DEFAULT_CLASS_HYPHEN: Self = b'-';
    const DEFAULT_CLASS_NEGATE: Self = b'!';
    const DEFAULT_CLASS_OPEN: Self = b'[';

    #[inline]
    fn match_one_case_sensitive(first: Self, second: Self) -> bool {
        first == second
    }

    #[inline]
    fn match_one_case_insensitive(first: Self, second: Self) -> bool {
        first.eq_ignore_ascii_case(&second)
    }

    #[inline]
    fn match_range_case_sensitive(token: Self, low: Self, high: Self) -> bool {
        low <= token && token <= high
    }

    #[inline]
    fn match_range_case_insensitive(token: Self, low: Self, high: Self) -> bool {
        if low <= token && token <= high {
            true
        } else if !token.is_ascii_alphabetic() {
            false
        } else {
            is_in_ascii_range_case_insensitive(token, low, high)
        }
    }
}

impl Wildcard for char {
    const DEFAULT_ANY: Self = '*';
    const DEFAULT_ESCAPE: Self = '\\';
    const DEFAULT_ONE: Self = '?';
    const DEFAULT_CLASS_CLOSE: Self = ']';
    const DEFAULT_CLASS_HYPHEN: Self = '-';
    const DEFAULT_CLASS_NEGATE: Self = '!';
    const DEFAULT_CLASS_OPEN: Self = '[';

    #[inline]
    fn match_one_case_insensitive(first: Self, second: Self) -> bool {
        first.eq_ignore_ascii_case(&second)
    }

    #[inline]
    fn match_one_case_sensitive(first: Self, second: Self) -> bool {
        first == second
    }

    #[inline]
    fn match_range_case_sensitive(token: Self, low: Self, high: Self) -> bool {
        low <= token && token <= high
    }

    #[inline]
    fn match_range_case_insensitive(token: Self, low: Self, high: Self) -> bool {
        if low <= token && token <= high {
            true
        } else if !(low.is_ascii() && high.is_ascii() && token.is_ascii_alphabetic()) {
            false
        } else {
            is_in_ascii_range_case_insensitive(token as u8, low as u8, high as u8)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// The main dowild functions
////////////////////////////////////////////////////////////////////////////////

/// Returns `true` if the wildcard pattern matches the `haystack`.
///
/// Allowed wildcard characters are `*` to match any amount of characters and `?` to match
/// exactly one character.
///
/// This is the basic algorithm without customization options to provide the best performance.
/// If you need [`Options`] you can use [`dowild_with`].
///
/// Instead of using this function, match directly on strings, u8 slices, ... without
/// performance loss, if you bring the [`DoWild`] trait in scope.
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
/// use simplematch::DoWild;
///
/// assert_eq!("*bc".dowild("aaabc"), true);
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
                // This (expensive) case is ensured to be entered only once per `wildcard_any` (or
                // multiple consecutive `wildcard_any`) character in the pattern. This allows us to
                // perform optimizations which would be otherwise not worth it. Note that every
                // increment of the indices in this match case also increments the respective
                // `next_*` index in the end.
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
                        // The end of the haystack might not yet be reached but for example `*????`
                        // matches anything.
                        if p_idx >= pattern.len() {
                            return true;
                        }
                    } else {
                        // Advancing the haystack and indirectly the `next_h_idx` counter to the
                        // first match significantly enhances the overall performance.
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
                    // after `wildcard_one` if it is the character after `wildcard_any`). However, we
                    // need to ensure in this match case that `next_p_idx` is not out of bounds.
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
        // If `true`, we need to reset. Therefore, this statement can be entered multiple times per
        // `wildcard_any`, so we need to be more careful with optimizations here than in the
        // `wildcard_any` match case above.
        if has_seen_wildcard_any && next_h_idx < haystack.len() {
            p_idx = next_p_idx;
            next_h_idx += 1;

            // We don't enter the `wildcard_any` match case in the big loop again, so we have to
            // apply this optimization from above here again, if applicable. This check let's the
            // compiler optimize the loop better than without the check although p_idx can't be
            // out of bounds here.
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

/// Returns `true` if the wildcard pattern matches the `haystack`. This method can be
/// customized with [`Options`].
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
/// or more conveniently match directly on a string bringing the [`DoWild`] trait in
/// scope.
///
/// ```rust
/// use simplematch::{DoWild, Options};
///
/// assert_eq!(
///     "%bc".dowild_with("aaabc", Options::default().wildcard_any_with(b'%')),
///     true
/// );
/// ```
#[must_use]
pub fn dowild_with<T>(pattern: &[T], haystack: &[T], options: Options<T>) -> bool
where
    T: Wildcard + Ord,
{
    if options.case_sensitive {
        dowild_with_worker(
            pattern,
            haystack,
            options,
            T::match_one_case_sensitive,
            T::match_range_case_sensitive,
        )
    } else {
        dowild_with_worker(
            pattern,
            haystack,
            options,
            T::match_one_case_insensitive,
            T::match_range_case_insensitive,
        )
    }
}

/// This method has the same structure like [`dowild`] but can apply [`Options`]
///
/// Customizability has a price performance-wise, so this method is by nature slower than
/// [`dowild`].
#[inline]
#[allow(clippy::too_many_lines)]
fn dowild_with_worker<F, G, T>(
    pattern: &[T],
    haystack: &[T],
    options: Options<T>,
    match_one: F,
    match_range: G,
) -> bool
where
    T: Wildcard + Ord,
    F: Fn(T, T) -> bool + Copy,
    G: Fn(T, T, T) -> bool + Copy,
{
    let Options {
        class_negate,
        is_classes_enabled,
        is_escape_enabled,
        wildcard_any,
        wildcard_escape,
        wildcard_one,
        ..
    } = options;

    let is_wildcard_any = |token: T| token == wildcard_any;
    let is_wildcard_one = |token: T| token == wildcard_one;
    let is_escape = |token: T| is_escape_enabled && token == wildcard_escape;
    let is_class_open = |token: T| is_classes_enabled && token == T::DEFAULT_CLASS_OPEN;

    let is_special = |token: T| {
        token == wildcard_any
            || token == wildcard_one
            || token == wildcard_escape
            || (is_classes_enabled && token == T::DEFAULT_CLASS_OPEN)
    };

    let is_valid_class_or_escape = |token: T, p_idx: usize, invalid_class_idx: usize| {
        (is_classes_enabled && token == T::DEFAULT_CLASS_OPEN && p_idx < invalid_class_idx)
            || (is_escape_enabled && token == wildcard_escape)
    };

    let mut p_idx = 0;
    let mut h_idx = 0;

    let mut next_p_idx = 0;
    let mut next_h_idx = 0;

    // There are no allocations, yet. `CharacterClasses` allocate on first use.
    let mut classes = CharacterClasses::new();

    let mut has_seen_wildcard_any = false;
    let mut invalid_class_idx = usize::MAX;

    while p_idx < pattern.len() || h_idx < haystack.len() {
        if p_idx < pattern.len() {
            match pattern[p_idx] {
                c if is_wildcard_any(c) => {
                    has_seen_wildcard_any = true;
                    p_idx += 1;

                    while p_idx < pattern.len() && is_wildcard_any(pattern[p_idx]) {
                        p_idx += 1;
                    }
                    if p_idx >= pattern.len() {
                        return true;
                    }

                    let next_c = pattern[p_idx];
                    #[allow(clippy::else_if_without_else)]
                    if is_wildcard_one(next_c) {
                        while h_idx < haystack.len() {
                            p_idx += 1;
                            h_idx += 1;
                            if !(p_idx < pattern.len() && is_wildcard_one(pattern[p_idx])) {
                                break;
                            }
                        }
                        if p_idx >= pattern.len() {
                            return true;
                        }
                    } else if !is_valid_class_or_escape(next_c, p_idx, invalid_class_idx) {
                        while h_idx < haystack.len() && !match_one(haystack[h_idx], next_c) {
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
                c if is_wildcard_one(c) => {
                    if h_idx < haystack.len() {
                        p_idx += 1;
                        h_idx += 1;
                        continue;
                    }
                }
                // Handling of the escape character. If it is the last character in the pattern, it
                // can only stand for itself.
                c if is_escape(c) && p_idx + 1 < pattern.len() => {
                    if h_idx < haystack.len() {
                        let next_c = pattern[p_idx + 1];
                        let h = haystack[h_idx];

                        #[allow(clippy::else_if_without_else)]
                        if is_special(next_c) && h == next_c {
                            p_idx += 2;
                            h_idx += 1;
                            continue;
                        } else if !is_special(next_c) && h == wildcard_escape {
                            p_idx += 1;
                            h_idx += 1;
                            continue;
                        }
                    }
                }
                // Handle character classes. To avoid parsing the same classes multiple times on
                // reset, every class including the invalid ones are stored in a container. However,
                // classes that are outside of the possible index don't need to be considered anymore
                // and are pruned.
                c if is_class_open(c) && p_idx < invalid_class_idx && p_idx + 1 < pattern.len() => {
                    if h_idx < haystack.len() {
                        let class = if has_seen_wildcard_any {
                            // Try to get rid of classes outside of the possible index
                            classes.prune(next_p_idx);
                            BorrowedOrOwned::Borrowed(classes.get_or_add(
                                p_idx,
                                pattern,
                                class_negate,
                            ))
                        } else {
                            // There's no need to store character classes as long as we don't require
                            // to reset.
                            BorrowedOrOwned::Owned(CharacterClasses::parse(
                                p_idx,
                                pattern,
                                class_negate,
                            ))
                        };

                        // Try to match this class. If it is an invalid class, we can interpret the
                        // opening bracket character literally and the rest of the pattern as if
                        // there is no class. If the class is valid and matched, we can advance as
                        // usual, otherwise we need to reset.
                        #[allow(clippy::else_if_without_else)]
                        if let Some(is_match) =
                            class.try_match(haystack[h_idx], match_one, match_range)
                        {
                            p_idx += class.len();
                            if is_match {
                                h_idx += 1;
                                continue;
                            }
                        } else {
                            invalid_class_idx = class.as_ref().start;
                            // A small shortcut to avoid the big loop and enter the generic
                            // character case.
                            if match_one(haystack[h_idx], T::DEFAULT_CLASS_OPEN) {
                                p_idx += 1;
                                h_idx += 1;
                                continue;
                            }
                        }
                    }
                }
                c => {
                    if h_idx < haystack.len() && match_one(haystack[h_idx], c) {
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
                && !is_valid_class_or_escape(pattern[p_idx], p_idx, invalid_class_idx)
            {
                while next_h_idx < haystack.len() && !match_one(haystack[next_h_idx], pattern[p_idx])
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

/// Returns true if the `token` is in the case insensitive inclusive range from `low` to `high`
///
/// `token` has to be ascii alphabetic character.
///
/// This function can be counter-intuitive, for example for `A-j` and the token `z`, this
/// function returns `true`. However, this is how regex engines (tested with python, go, the
/// regex crate, ...) usually evaluate it.
#[inline]
const fn is_in_ascii_range_case_insensitive(token: u8, low: u8, high: u8) -> bool {
    const ASCII_CASE_MASK: u8 = 0b0010_0000;

    if token.is_ascii_lowercase() {
        let token_uppercase = token ^ ASCII_CASE_MASK;
        low <= token_uppercase && token_uppercase <= high
    // Since token is alphabetic it is an uppercase character
    } else {
        let token_lowercase = token | ASCII_CASE_MASK;
        low <= token_lowercase && token_lowercase <= high
    }
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
        if case_sensitive {
            assert_eq!(Wildcard::match_one_case_sensitive(first, second), expected);
            assert_eq!(
                Wildcard::match_one_case_sensitive(first as char, second as char),
                expected
            );
        } else {
            assert_eq!(
                Wildcard::match_one_case_insensitive(first, second),
                expected
            );
            assert_eq!(
                Wildcard::match_one_case_insensitive(first as char, second as char),
                expected
            );
        }
    }

    #[rstest]
    #[case::all_the_same(b'j', b'j', b'j', true)]
    #[case::low_is_higher_high_is_same(b'j', b'k', b'j', false)]
    #[case::low_is_lower_high_is_same(b'j', b'i', b'j', true)]
    #[case::high_is_lower_low_is_same(b'j', b'k', b'i', false)]
    #[case::high_is_higher_low_is_same(b'j', b'j', b'k', true)]
    #[case::non_alpha_when_false(b'#', b'*', b']', false)]
    #[case::non_alpha_when_true(b'+', b'*', b']', true)]
    #[case::only_token_alpha(b'a', b'*', b']', false)]
    #[case::only_token_big_alpha(b'A', b'*', b']', true)]
    #[case::between_alphabetic(b']', b'*', b'B', false)]
    fn impl_wildcard_match_range_when_case_sensitive(
        #[case] token: u8,
        #[case] low: u8,
        #[case] high: u8,
        #[case] expected: bool,
    ) {
        assert_eq!(
            Wildcard::match_range_case_sensitive(token, low, high),
            expected
        );
        assert_eq!(
            Wildcard::match_range_case_sensitive(token as char, low as char, high as char),
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
    #[case::big_a_to_j(b'z', b'A', b'j', true)]
    #[case::non_alpha_when_false(b'#', b'*', b']', false)]
    #[case::control_when_false(b'\x1f', b'*', b']', false)]
    #[case::non_alpha_when_true(b'+', b'*', b']', true)]
    #[case::only_token_alpha(b'a', b'*', b']', true)]
    #[case::only_token_big_alpha(b'A', b'*', b']', true)]
    #[case::between_alphabetic(b']', b'*', b'B', false)]
    fn impl_wildcard_match_range_when_case_insensitive(
        #[case] token: u8,
        #[case] low: u8,
        #[case] high: u8,
        #[case] expected: bool,
    ) {
        assert_eq!(
            Wildcard::match_range_case_insensitive(token, low, high),
            expected
        );
        assert_eq!(
            Wildcard::match_range_case_insensitive(token as char, low as char, high as char),
            expected
        );
    }
}
