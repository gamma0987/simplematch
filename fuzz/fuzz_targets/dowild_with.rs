#![no_main]

use common::{pattern_to_regex, PatternOptions};
use libfuzzer_sys::fuzz_target;
use simplematch::{dowild_with, Options};

fuzz_target!(|data: (&[u8], &[u8], PatternOptions)| {
    let (pattern, haystack, options) = (
        std::str::from_utf8(data.0),
        std::str::from_utf8(data.1),
        data.2,
    );

    if let (Ok(pattern), Ok(haystack)) = (pattern, haystack) {
        if let Ok(regex) = pattern_to_regex(pattern, options) {
            assert_eq!(
                dowild_with(
                    pattern.as_bytes(),
                    haystack.as_bytes(),
                    Options::from(options)
                ),
                regex.is_match(haystack.as_bytes()),
                "The following should match:\npattern: '{}'\nbytes: '{:?}'\nand regex: \
                 '{}'\nbytes: '{:?}'\nwith options: '{:?}'\nshould match haystack: '{}'\nbytes: \
                 '{:?}'",
                pattern,
                pattern.as_bytes(),
                regex,
                regex.as_str().as_bytes(),
                options,
                haystack,
                haystack.as_bytes()
            );
        }
    }
});
