//! Copyright 2018 IBM Corporation
//! Copyright 2025 gamma0987
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//!     http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.
//!
//! Modifications by gamma0987. These tests originate from
//! <https://developforperformance.com/MatchingWildcards_AnImprovedAlgorithmForBigData.html> and were
//! ported to rust. Any additional tests in this file originate from
//! <https://research.swtch.com/glob>.
use rstest::rstest;
use simplematch::{dowild, dowild_with, Options};

// spell-checker: disable
#[rstest]
#[case::star_at_end("Hi*", "Hi", true)]
#[case::mismatch_after_star("ab*d", "abc", false)]
#[case::repeat_0("*ccd", "abcccd", true)]
#[case::repeat_1("*issip*ss*", "mississipissippi", true)]
#[case::repeat_2("xxxx*zzy*fffff", "xxxx*zzzzzzzzy*f", false)]
#[case::repeat_3("xxx*zzy*f", "xxxx*zzzzzzzzy*f", true)]
#[case::repeat_4("xxxx*zzy*fffff", "xxxxzzzzzzzzyf", false)]
#[case::repeat_5("xxxx*zzy*f", "xxxxzzzzzzzzyf", true)]
#[case::repeat_6("xy*z*xyz", "xyxyxyzyxyz", true)]
#[case::repeat_7("*sip*", "mississippi", true)]
#[case::repeat_8("xy*xyz", "xyxyxyxyz", true)]
#[case::repeat_9("mi*sip*", "mississippi", true)]
#[case::repeat_10("*abac*", "ababac", true)]
#[case::repeat_11("*abac*", "ababac", true)]
#[case::repeat_12("a*zz*", "aaazz", true)]
#[case::repeat_13("*12*23", "a12b12", false)]
#[case::repeat_14("a12b", "a12b12", false)]
#[case::repeat_15("*12*12*", "a12b12", true)]
#[case::star_in_tame_0("*", "*", true)]
#[case::star_in_tame_1("a*b", "a*abab", true)]
#[case::star_in_tame_2("a*", "a*r", true)]
#[case::star_in_tame_3("a*aar", "a*ar", false)]
#[case::double_wildcard_0("XY*Z*XYz", "XYXYXYZYXYz", true)]
#[case::double_wildcard_1("*SIP*", "missisSIPpi", true)]
#[case::double_wildcard_2("*issip*PI", "mississipPI", true)]
#[case::double_wildcard_3("xy*xyz", "xyxyxyxyz", true)]
#[case::double_wildcard_4("mi*sip*", "miSsissippi", true)]
#[case::double_wildcard_5("mi*Sip*", "miSsissippi", false)]
#[case::double_wildcard_6("*Abac*", "abAbac", true)]
#[case::double_wildcard_7("*Abac*", "abAbac", true)]
#[case::double_wildcard_8("a*zz*", "aAazz", true)]
#[case::double_wildcard_9("*12*23", "A12b12", false)]
#[case::double_wildcard_10("*12*12*", "a12B12", true)]
#[case::double_wildcard_11("*oWn*", "oWn", true)]
#[case::no_wildcard_0("bLah", "bLah", true)]
#[case::no_wildcard_1("bLaH", "bLah", false)]
#[case::mixed_wildcard_0("*?", "a", true)]
#[case::mixed_wildcard_1("*?", "ab", true)]
#[case::mixed_wildcard_2("*?", "abc", true)]
#[case::more_mixed_wildcard_0("??", "a", false)]
#[case::more_mixed_wildcard_1("?*?", "ab", true)]
#[case::more_mixed_wildcard_2("*?*?*", "ab", true)]
#[case::more_mixed_wildcard_3("?**?*?", "abc", true)]
#[case::more_mixed_wildcard_4("?**?*&?", "abc", false)]
#[case::more_mixed_wildcard_5("?b*??", "abcd", true)]
#[case::more_mixed_wildcard_6("?a*??", "abcd", false)]
#[case::more_mixed_wildcard_7("?**?c?", "abcd", true)]
#[case::more_mixed_wildcard_8("?**?d?", "abcd", false)]
#[case::more_mixed_wildcard_9("?*b*?*d*?", "abcde", true)]
#[case::single_wildcard_0("bL?h", "bLah", true)]
#[case::single_wildcard_1("bLa?", "bLaaa", false)]
#[case::single_wildcard_2("bLa?", "bLah", true)]
#[case::single_wildcard_3("?Lah", "bLaH", false)]
#[case::single_wildcard_4("?LaH", "bLaH", true)]
#[case::many_wildcards_0(
    "a*a*a*a*a*a*aa*aaa*a*a*b",
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab",
    true
)]
#[case::many_wildcards_0("*a*b*ba*ca*a*aa*aaa*fa*ga*b*", "abababababababababababababababababababaacacacacacacacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab", true)]
#[case::many_wildcards_1("*a*b*ba*ca*a*x*aaa*fa*ga*b*", "abababababababababababababababababababaacacacacacacacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab", false)]
#[case::many_wildcards_2("*a*b*ba*ca*aaaa*fa*ga*gggg*b*", "abababababababababababababababababababaacacacacacacacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab", false)]
#[case::many_wildcards_3("*a*b*ba*ca*aaaa*fa*ga*ggg*b*", "abababababababababababababababababababaacacacacacacacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab", true)]
#[case::many_wildcards_4("*aabbaa*a*", "aaabbaabbaab", true)]
#[case::many_wildcards_5(
    "a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*",
    "a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*",
    true
)]
#[case::many_wildcards_6("*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*", "aaaaaaaaaaaaaaaaa", true)]
#[case::many_wildcards_7("*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*", "aaaaaaaaaaaaaaaa", false)]
#[case::many_wildcards_8(
    "abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*",
    "abc*abcd*abcde*abcdef*abcdefg*abcdefgh*abcdefghi*abcdefghij*abcdefghijk*abcdefghijkl*\
     abcdefghijklm*abcdefghijklmn",
    false
)]
#[case::many_wildcards_9(
    "abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*",
    "abc*abcd*abcde*abcdef*abcdefg*abcdefgh*abcdefghi*abcdefghij*abcdefghijk*abcdefghijkl*\
     abcdefghijklm*abcdefghijklmn",
    true
)]
#[case::many_wildcards_10("abc*abc*abc*abc*abc", "abc*abcd*abcd*abc*abcd", false)]
#[case::many_wildcards_11(
    "abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abcd",
    "abc*abcd*abcd*abc*abcd*abcd*abc*abcd*abc*abc*abcd",
    true
)]
#[case::many_wildcards_12("********a********b********c********", "abc", true)]
#[case::many_wildcards_13("abc", "********a********b********c********", false)]
#[case::many_wildcards_14("********a********b********b********", "abc", false)]
#[case::many_wildcards_15("***a*b*c***", "*abc*", true)]
#[case::more_tests_0("?", "", false)]
#[case::more_tests_1("*?", "", false)]
#[case::more_tests_2("", "", true)]
#[case::more_tests_3("", "a", false)]
// spell-checker: enable
fn test_dowild(#[case] pattern: String, #[case] haystack: String, #[case] expected: bool) {
    assert_eq!(
        dowild(pattern.as_bytes(), haystack.as_bytes()),
        expected,
        "Assert dowild"
    );

    // case_insensitive matching is not included here
    assert_eq!(
        dowild_with(pattern.as_bytes(), haystack.as_bytes(), Options::new()),
        expected,
        "Assert dowild_with and default options"
    );
    assert_eq!(
        dowild_with(
            pattern.as_bytes(),
            haystack.as_bytes(),
            Options::new().enable_escape(true)
        ),
        expected,
        "Assert dowild_with and enable_escape"
    );
    assert_eq!(
        dowild_with(
            pattern.replace('\\', "&").as_bytes(),
            haystack.as_bytes(),
            Options::new().enable_escape_with(b'&')
        ),
        expected,
        "Assert dowild_with and enable_escape_with '&'"
    );
    assert_eq!(
        dowild_with(
            pattern.as_bytes(),
            haystack.as_bytes(),
            Options::new().enable_classes(true)
        ),
        expected,
        "Assert dowild_with and enable_ranges"
    );
    assert_eq!(
        dowild_with(
            pattern.replace('!', "^").as_bytes(),
            haystack.as_bytes(),
            Options::new().enable_classes_with(b'^')
        ),
        expected,
        "Assert dowild_with and enable_ranges_with '^'"
    );
    assert_eq!(
        dowild_with(
            pattern.replace('*', "%").as_bytes(),
            haystack.as_bytes(),
            Options::new().wildcard_any_with(b'%')
        ),
        expected,
        "Assert dowild_with and wildcard_any_with '%'"
    );
    assert_eq!(
        dowild_with(
            pattern.replace('?', "_").as_bytes(),
            haystack.as_bytes(),
            Options::new().wildcard_one_with(b'_')
        ),
        expected,
        "Assert dowild_wild and wildcard_one_with '_'"
    );
}

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
fn basic_tests_dowild_and_dowild_options(
    #[case] pattern: String,
    #[case] haystack: String,
    #[case] expected: bool,
) {
    assert_eq!(
        dowild(pattern.as_bytes(), haystack.as_bytes()),
        expected,
        "Assert dowild"
    );

    assert_eq!(
        dowild_with(pattern.as_bytes(), haystack.as_bytes(), Options::new()),
        expected,
        "Assert dowild_with and default options"
    );
    assert_eq!(
        dowild_with(
            pattern.as_bytes(),
            haystack.as_bytes(),
            Options::new().case_insensitive(true)
        ),
        expected,
        "Assert dowild_with and case_insensitive"
    );
    assert_eq!(
        dowild_with(
            pattern.as_bytes(),
            haystack.as_bytes(),
            Options::new().enable_escape(true)
        ),
        expected,
        "Assert dowild_with and enable_escape"
    );
    assert_eq!(
        dowild_with(
            pattern.replace('\\', "&").as_bytes(),
            haystack.as_bytes(),
            Options::new().enable_escape_with(b'&')
        ),
        expected,
        "Assert dowild_with and enable_escape_with '&'"
    );
    assert_eq!(
        dowild_with(
            pattern.as_bytes(),
            haystack.as_bytes(),
            Options::new().enable_classes(true)
        ),
        expected,
        "Assert dowild_with and enable_ranges"
    );
    assert_eq!(
        dowild_with(
            pattern.replace('!', "^").as_bytes(),
            haystack.as_bytes(),
            Options::new().enable_classes_with(b'^')
        ),
        expected,
        "Assert dowild_with and enable_ranges_with '^'"
    );
    assert_eq!(
        dowild_with(
            pattern.replace('*', "%").as_bytes(),
            haystack.as_bytes(),
            Options::new().wildcard_any_with(b'%')
        ),
        expected,
        "Assert dowild_with and wildcard_any_with '%'"
    );
    assert_eq!(
        dowild_with(
            pattern.replace('?', "_").as_bytes(),
            haystack.as_bytes(),
            Options::new().wildcard_one_with(b'_')
        ),
        expected,
        "Assert dowild_wild and wildcard_one_with '_'"
    );
}
