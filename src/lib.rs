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
//! instructions than the original algorithm; tested with random small and big data.

// spell-checker: ignore aaabc fooa Krauss

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

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
use core::fmt::Display;
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
/// use simplematch::SimpleMatch;
///
/// assert_eq!("foobar".dowild("foo*"), true);
/// ```
pub trait SimpleMatch<T>
where
    T: Wildcard,
{
    /// Matches this `haystack` against the specified `pattern` using simple wildcard rules.
    ///
    /// See [`dowild`] for more details.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simplematch::SimpleMatch;
    ///
    /// assert_eq!("foobar".dowild("foo*"), true);
    /// ```
    #[must_use]
    fn dowild(&self, pattern: Self) -> bool;

    /// Matches this haystack against the specified `pattern` with customizable [`Options`].
    ///
    /// See [`dowild_with`] for more details.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use simplematch::{Options, SimpleMatch};
    ///
    /// assert_eq!(
    ///     "foobar".dowild_with("foo*", Options::default().case_insensitive(true)),
    ///     true
    /// );
    /// ```
    #[must_use]
    fn dowild_with(&self, pattern: Self, options: Options<T>) -> bool;
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

// Represents a character class
#[derive(Debug)]
struct CharacterClass<T> {
    /// If `None`, the character class is invalid.
    class: Option<Class<T>>,
    /// The end index in the pattern
    end: usize,
    /// The start index in the pattern
    start: usize,
}

#[derive(Debug)]
struct CharacterClasses<T>(VecDeque<CharacterClass<T>>);

#[derive(Debug)]
enum Class<T> {
    Positive(Vec<ClassKind<T>>),
    Negative(Vec<ClassKind<T>>),
}

#[derive(Debug)]
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

impl<T> CharacterClass<T>
where
    T: Wildcard + Ord,
{
    #[inline]
    const fn new(class: Option<Class<T>>, start: usize, end: usize) -> Self {
        Self { class, end, start }
    }

    #[inline]
    const fn new_invalid(start: usize, end: usize) -> Self {
        Self::new(None, start, end)
    }

    #[inline]
    const fn len(&self) -> usize {
        self.end - self.start + 1
    }

    fn parse(start: usize, pattern: &[T], class_negate: T) -> Self {
        // The first character of a range is always the opening bracket
        let mut p_idx = start + 1;
        if p_idx + 2 > pattern.len() {
            // The pattern is too short to produce a valid range
            return Self::new_invalid(start, p_idx + 1);
        }

        let mut class = if pattern[p_idx] == class_negate {
            p_idx += 1;
            Class::new_negative(p_idx, pattern.len())
        } else {
            Class::new_positive(p_idx, pattern.len())
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
    #[inline]
    fn new() -> Self {
        Self(VecDeque::new())
    }

    #[inline]
    fn get(&self, index: usize) -> Option<&CharacterClass<T>> {
        self.0.iter().find(|r| r.start == index)
    }

    fn get_or_add(&mut self, start: usize, pattern: &[T], class_negate: T) -> &CharacterClass<T> {
        if self.0.capacity() == 0 {
            self.0.reserve(pattern.len() - start);
        }
        if let Some(last) = self.0.back() {
            if last.start >= start {
                return self.get(start).unwrap();
            }
        }

        let class = CharacterClass::parse(start, pattern, class_negate);
        self.0.push_back(class);
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

impl<T> Class<T>
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
    fn push(&mut self, kind: ClassKind<T>) {
        match self {
            Self::Positive(kinds) | Self::Negative(kinds) => kinds.push(kind),
        }
    }

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
/// performance loss, if you bring the [`SimpleMatch`] trait in scope.
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
        is_classes_enabled,
        class_negate,
        wildcard_any,
        wildcard_escape,
        wildcard_one,
        is_escape_enabled,
        ..
    } = options;

    let is_special = |token: T| {
        token == wildcard_any
            || token == wildcard_one
            || token == wildcard_escape
            || (is_classes_enabled && token == T::DEFAULT_CLASS_OPEN)
    };

    let mut p_idx = 0;
    let mut h_idx = 0;

    let mut next_p_idx = 0;
    let mut next_h_idx = 0;

    let mut classes = CharacterClasses::new();

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
                        || (is_classes_enabled && next_c == T::DEFAULT_CLASS_OPEN))
                    {
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
                c if is_classes_enabled
                    && c == T::DEFAULT_CLASS_OPEN
                    && p_idx + 1 < pattern.len() =>
                {
                    if h_idx < haystack.len() {
                        if has_seen_wildcard_any {
                            classes.prune(next_p_idx);
                        }

                        let class = classes.get_or_add(p_idx, pattern, class_negate);
                        #[allow(clippy::else_if_without_else)]
                        if let Some(is_match) =
                            class.try_match(haystack[h_idx], match_one, match_range)
                        {
                            p_idx += class.len();
                            if is_match {
                                h_idx += 1;
                                continue;
                            }
                        } else if match_one(haystack[h_idx], T::DEFAULT_CLASS_OPEN) {
                            p_idx += 1;
                            h_idx += 1;
                            continue;
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
                && !(is_classes_enabled && pattern[p_idx] == T::DEFAULT_CLASS_OPEN)
                && !(is_escape_enabled && pattern[p_idx] == wildcard_escape)
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
