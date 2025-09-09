#![no_main]

use common::{pattern_to_regex, PatternOptions};
use libfuzzer_sys::fuzz_target;
use simplematch::dowild;

fuzz_target!(|data: (&[u8], &[u8])| {
    if let (Ok(pattern), Ok(haystack)) = (std::str::from_utf8(data.0), std::str::from_utf8(data.1)) {
        if let Ok(regex) = pattern_to_regex(pattern, PatternOptions::default()) {
            assert_eq!(
                dowild(pattern.as_bytes(), haystack.as_bytes()),
                regex.is_match(haystack.as_bytes()),
                "The following should match:\npattern: '{}'\nbytes: '{:?}'\nand regex: \
                 '{}'\nbytes: '{:?}'\nshould match haystack: '{}'\nbytes: '{:?}'",
                pattern,
                pattern.as_bytes(),
                regex,
                regex.as_str().as_bytes(),
                haystack,
                haystack.as_bytes()
            );
        }
    }
});
