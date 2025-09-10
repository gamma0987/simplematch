//! The tests

use rstest::rstest;
use simplematch::{dowild, dowild_with, Options, SimpleMatch};

// These are mostly tests from https://research.swtch.com/glob
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
#[case::for_debug_0("a*?b", format!("{}b", "a".repeat(100)), true)]
#[case::for_debug_1("a*?b", "aaab", true)]
#[case::multi_a_star("a*a*a*a*b", "a".repeat(100), false)]
#[case::star_x("*x", "xxx", true)]
#[case::multi_star("x******x", "xx", true)]
// cspell: enable
fn basic_test_dowild(#[case] pattern: String, #[case] haystack: String, #[case] expected: bool) {
    assert_eq!(dowild(pattern.as_bytes(), haystack.as_bytes()), expected);
    assert_eq!(
        dowild_with(
            pattern.as_bytes(),
            haystack.as_bytes(),
            Options::new().enable_escape(true)
        ),
        expected
    );
    assert_eq!(
        dowild_with(
            pattern.as_bytes(),
            haystack.as_bytes(),
            Options::new().case_insensitive(true).enable_escape(true)
        ),
        expected
    );
    assert_eq!(
        dowild_with(pattern.as_bytes(), haystack.as_bytes(), Options::new()),
        expected
    );
    assert_eq!(
        dowild_with(
            pattern.as_bytes(),
            haystack.as_bytes(),
            Options::new().case_insensitive(true)
        ),
        expected
    );
}

#[test]
fn impl_for_str() {
    assert_eq!("abc".dowild("a*c"), true);
    assert_eq!(
        "abc".dowild_with("a*c", Options::new().enable_escape(true)),
        true
    );
}

#[test]
fn impl_for_string() {
    assert_eq!(String::from("abc").as_str().dowild("a*c"), true);
    assert_eq!(
        String::from("abc")
            .as_str()
            .dowild_with("a*c", Options::new().enable_escape(true)),
        true
    );
}

#[test]
fn impl_for_u8_slice() {
    assert_eq!(b"abc".as_slice().dowild("a*c".as_bytes()), true);
    assert_eq!(
        b"abc"
            .as_slice()
            .dowild_with("a*c".as_bytes(), Options::new().enable_escape(true)),
        true
    );
}

#[test]
fn impl_for_u8_vec() {
    assert_eq!(b"abc".to_vec().dowild(b"a*c".to_vec()), true);
    assert_eq!(
        b"abc"
            .to_vec()
            .dowild_with(b"a*c".to_vec(), Options::new().enable_escape(true)),
        true
    );
}

#[test]
fn impl_for_char_slice() {
    assert_eq!(
        ['a', 'b', 'c']
            .as_slice()
            .dowild(['a', '*', 'c'].as_slice()),
        true
    );
    assert_eq!(
        ['a', 'b', 'c'].as_slice().dowild_with(
            ['a', '*', 'c'].as_slice(),
            Options::new().enable_escape(true)
        ),
        true
    );
}

#[test]
fn impl_for_char_vec() {
    assert_eq!(
        ['a', 'b', 'c'].to_vec().dowild(['a', '*', 'c'].to_vec()),
        true
    );
    assert_eq!(
        ['a', 'b', 'c']
            .to_vec()
            .dowild_with(['a', '*', 'c'].to_vec(), Options::new().enable_escape(true)),
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
    let options = Options::new().enable_escape(true);
    assert_eq!(
        dowild_with(pattern.as_bytes(), haystack.as_bytes(), options),
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
        dowild_with(pattern.as_bytes(), haystack.as_bytes(), options),
        expected
    );
}

#[rstest]
#[case::empty("[]", &["[]"], true)]
#[case::open_without_close("[abc", &["[abc"], true)]
#[case::close_bracket_end("a]", &["a]"], true)]
#[case::open_bracket_end("a[", &["a["], true)]
#[case::just_a("[a]", &["a"], true)]
#[case::multi_a("[aaaaaa]", &["a"], true)]
#[case::just_negative("[!]", &["!"], false)]
#[case::negative_a("[!a]", &["a"], false)]
#[case::literal_negative("[a!]", &["a", "!"], true)]
#[case::literal_star("[*]", &["*"], true)]
#[case::literal_question_mark("[?]", &["?"], true)]
#[case::literal_backslash(r"[\]", &[r"\"], true)]
#[case::close_bracket_first("[]]", &["]"], true)]
#[case::close_bracket_first_negative_open("[]![]", &["]", "!", "["], true)]
#[case::negative_close_bracket("[!]]", &["]"], false)]
#[case::multi_close_bracket("[]]]", &["]]"], true)]
#[case::open_bracket_first("[[]", &["["], true)]
#[case::multi_open_bracket("[[[]", &["["], true)]
#[case::close_bracket_minus("[]-]", &["]", "-"], true)]
#[case::minus_close_bracket("[-]]", &["-]"], true)]
#[case::just_minus("[-]", &["-"], true)]
#[case::negative_minus("[!-]", &["-"], false)]
#[case::a_minus("[a-]", &["-", "a"], true)]
#[case::minus_a("[-a]", &["-", "a"], true)]
#[case::range_same("[a-a]", &["a"], true)]
#[case::a_to_z("[a-z]", &["a", "m", "z"], true)]
#[case::negative_a_to_z("[!a-z]", &["a", "m", "z"], false)]
#[case::a_to_z_not_uppercase("[a-z]", &["A", "M", "Z"], false)]
#[case::z_to_a("[z-a]", &["a", "m", "z"], true)]
#[case::negative_z_to_a("[!z-a]", &["a", "m", "z"], false)]
#[case::multi_range("[a-zA-Z]", &["a", "m", "z", "A", "M", "Z"], true)]
#[case::negative_multi_range("[!a-zA-Z]", &["a", "m", "z", "A", "M", "Z"], false)]
#[case::multi_range_at_the_end("a[a-zA-Z]", &["aa", "az", "aA", "aZ"], true)]
#[case::multi_range_at_the_start("[a-zA-Z]z", &["az", "Az", "Zz"], true)]
#[case::multi_range_in_the_middle("a[a-zA-Z]z", &["aaz", "azz", "aAz", "aZz"], true)]
fn dowild_with_range_case_sensitive(
    #[case] pattern: String,
    #[case] haystacks: &[&str],
    #[case] expected: bool,
) {
    let options = Options::new().enable_ranges(true);
    let char_options = Options::new().enable_ranges(true);
    for haystack in haystacks {
        assert_eq!(
            dowild_with(pattern.as_bytes(), haystack.as_bytes(), options),
            expected,
            "haystack was: {haystack}",
        );
        assert_eq!(
            dowild_with(
                &pattern.chars().collect::<Vec<char>>(),
                &haystack.chars().collect::<Vec<char>>(),
                char_options
            ),
            expected,
            "char haystack was: {haystack}",
        );
    }
}

// From fuzzy tests
// cspell: disable
#[rstest]
#[case::fuzz_0("*[a]", &["cba"], true, true)]
#[case::fuzz_1("*[--$j-/]", &["*"], true, true)]
#[case::fuzz_2("[n", &["\0"], false, true)]
#[case::fuzz_4("*[x", &[".[x"], true, true)]
#[case::fuzz_5("[.$--.j-.\\/j.]", &["A"], true, true)]
#[case::fuzz_6("[]G[a]", &["G"], true, true)]
#[case::fuzz_7("[--$j\0-\0--\0-\0]", &["#"], true, true)]
#[case::fuzz_8("*[--$j---\0\0\0\0]", &["/"], true, true)]
#[case::fuzz_9("[\0\0-\0]", &["-"], false, true)]
#[case::fuzz_10("[/-a]", &["]"], true, false)]
#[case::fuzz_11("[/-A]", &["]"], false, false)]
// the respective converted regex `is_match` was true, which is not correct!
#[case::fuzz_12("[/-j]", &["z"], false, false)]
#[case::fuzz_13("[]--]G", &["GG"], true, true)]
#[case::fuzz_14("[]-\0", &["5"], false, true)]
#[case::fuzz_14("[!]a]", &[","], true, true)]
// cspell: enable
fn dowild_with_range_and_wildcards(
    #[case] pattern: String,
    #[case] haystacks: &[&str],
    #[case] expected: bool,
    #[case] case_sensitive: bool,
) {
    let options = Options::new()
        .case_insensitive(!case_sensitive)
        .enable_ranges(true);
    let char_options = Options::new()
        .case_insensitive(!case_sensitive)
        .enable_ranges(true);
    for haystack in haystacks {
        assert_eq!(
            dowild_with(pattern.as_bytes(), haystack.as_bytes(), options),
            expected,
            "haystack was: {haystack}",
        );
        assert_eq!(
            dowild_with(
                &pattern.chars().collect::<Vec<char>>(),
                &haystack.chars().collect::<Vec<char>>(),
                char_options
            ),
            expected,
            "char haystack was: {haystack}",
        );
    }
}
