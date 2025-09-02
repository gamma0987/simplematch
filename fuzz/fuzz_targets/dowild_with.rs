#![no_main]

use libfuzzer_sys::fuzz_target;
use quickmatch::{dowild_with, Options};
use quickmatch_fuzz::pattern_to_regex;

fuzz_target!(|data: (&[u8], &[u8], Options)| {
    if let (Ok(pattern), Ok(haystack)) = (std::str::from_utf8(data.0), std::str::from_utf8(data.1)) {
        let options = data.2;

        if let Ok(regex) = pattern_to_regex(pattern, options) {
            assert_eq!(
                dowild_with(pattern, haystack, options),
                regex.is_match(haystack),
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
