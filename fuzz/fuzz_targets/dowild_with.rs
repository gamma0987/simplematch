#![no_main]

use common::{pattern_to_regex, PatternOptions, DEFAULT_ESCAPE};
use libfuzzer_sys::fuzz_target;
use simplematch::{dowild_with, Options};
use simplematch_fuzz::FuzzOptions;

fuzz_target!(|data: (&[u8], &[u8], FuzzOptions)| {
    let (pattern, haystack, fuzz_options) = (
        std::str::from_utf8(data.0),
        std::str::from_utf8(data.1),
        data.2,
    );

    if let (Ok(pattern), Ok(haystack)) = (pattern, haystack) {
        let mut pattern_options = PatternOptions {
            case_sensitive: fuzz_options.case_sensitive,
            // We switch off character classes if using case-insensitive matching. This is a
            // limitation of Regex. The case insensitive matching of regex seems to contain a bug
            // when using character classes. For example `"^[A-j]$"` matches `"z"` when Regex is
            // configured to match case-insensitive.
            is_ranges_enabled: fuzz_options.case_sensitive && fuzz_options.use_ranges,
            ..Default::default()
        };

        // Try to use characters which have no special meaning in regex
        if fuzz_options.use_other_negate {
            pattern_options.range_negate = Some(b'#');
        }
        if fuzz_options.use_other_wildcard_any {
            pattern_options.wildcard_any = Some(b'%');
        }
        if fuzz_options.use_other_wildcard_one {
            pattern_options.wildcard_one = Some(b'_');
        }
        if fuzz_options.enable_escape {
            pattern_options.wildcard_escape = if fuzz_options.use_other_wildcard_escape {
                Some(b'=')
            } else {
                Some(DEFAULT_ESCAPE)
            };
        }

        let simplematch_options = Options::from(pattern_options);
        if let Ok(regex) = pattern_to_regex(pattern, pattern_options) {
            assert_eq!(
                dowild_with(pattern.as_bytes(), haystack.as_bytes(), simplematch_options),
                regex.is_match(haystack.as_bytes()),
                "The following should hold:\nfuzzy options: '{:?}'\nand simplematch options: \
                 '{:?}'\npattern: '{}'\nbytes: '{:?}'\nregex: '{}'\nbytes: '{:?}'\nshould match \
                 haystack: '{}'\nbytes: '{:?}'",
                fuzz_options,
                simplematch_options,
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
