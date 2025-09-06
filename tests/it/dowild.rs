//! The tests

use quickmatch::{dowild, dowild_with, Options, QuickMatch, DEFAULT_ESCAPE};
use rstest::rstest;

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
            Options::new().enable_escape_with(DEFAULT_ESCAPE)
        ),
        expected
    );
    assert_eq!(
        dowild_with(
            pattern.as_str(),
            haystack.as_str(),
            Options::new()
                .case_insensitive(true)
                .enable_escape_with(DEFAULT_ESCAPE)
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
            Options::new().case_insensitive(true)
        ),
        expected
    );
}

#[test]
fn impl_for_str() {
    assert_eq!("abc".dowild("a*c"), true);
    assert_eq!(
        "abc".dowild_with("a*c", Options::new().enable_escape_with(DEFAULT_ESCAPE)),
        true
    );
}

#[test]
fn impl_for_string() {
    assert_eq!(String::from("abc").dowild("a*c"), true);
    assert_eq!(
        String::from("abc").dowild_with("a*c", Options::new().enable_escape_with(DEFAULT_ESCAPE)),
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
    let options = Options::new().enable_escape_with(DEFAULT_ESCAPE);
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
    let options = Options::new().enable_escape_with(b'\0');
    assert_eq!(
        dowild_with(pattern.as_str(), haystack.as_str(), options),
        expected
    );
}
