use std::hint::black_box;

use iai_callgrind::{
    library_benchmark, library_benchmark_group, main, Dhat, EntryPoint, LibraryBenchmarkConfig,
};
use simplematch::{dowild_with, Options};

const DEFAULT_OPTIONS: Options<u8> = Options::new();
const OPTIONS_WITH_CLASSES: Options<u8> = DEFAULT_OPTIONS.enable_classes(true);

#[library_benchmark]
#[benches::just_a(
    ("a", &"a".repeat(100)),
    (&"a".repeat(100), "a"),
    (&"a".repeat(100), &"a".repeat(100)),
)]
#[benches::mixed(
    ("abcd", "abcd"),
    ("abcd", &"abcd".repeat(25)),
    (&"abcd".repeat(25), "abcd"),
    (&"abcd".repeat(25), &"abcd".repeat(25)),
)]
// spell-checker: enable
fn bench_dowild_without_wildcards(pattern: &str, haystack: &str) -> bool {
    black_box(dowild_with(
        black_box(pattern.as_bytes()),
        black_box(haystack.as_bytes()),
        black_box(DEFAULT_OPTIONS),
    ))
}

// spell-checker: disable
#[library_benchmark]
#[benches::linearity(
    ("a*b", &format!("{}b", "a".repeat(100))),
    (&format!("{}b", "a*".repeat(50)), &format!("{}b", "a".repeat(100))),
    (&format!("{}b", "a*".repeat(100)), &format!("{}b", "a".repeat(100)))
)]
#[benches::multiple(
    ("a**b", &format!("{}b", "a".repeat(100))),
    (&format!("a{}b", "*".repeat(100)), &format!("{}b", "a".repeat(100))),
)]
#[benches::with_wildcard_one(
    ("a*?b", &format!("{}b", "a".repeat(100))),
    ("a*?bbbbb", &format!("{}bbbbbb", "a".repeat(100))),
    ("a*bbbb?b", &format!("{}bbbbcb", "a".repeat(100))),
    ("a*?????b", &format!("{}b", "a".repeat(100))),
)]
// spell-checker: enable
fn bench_dowild_with_wildcard_any(pattern: &str, haystack: &str) -> bool {
    black_box(dowild_with(
        black_box(pattern.as_bytes()),
        black_box(haystack.as_bytes()),
        black_box(DEFAULT_OPTIONS),
    ))
}

#[library_benchmark(
    config = LibraryBenchmarkConfig::default()
        .tool(Dhat::default()
            .entry_point(
                EntryPoint::Custom("simplematch::dowild_with".to_owned())
            )
        )
)]
#[benches::just_opening_brackets(
    ("[", "["),
    ("[[[", "[[["),
    (&"[".repeat(100), &"[".repeat(100)),
    (&"[a".repeat(50), &"[a".repeat(50))
)]
#[benches::valid(
    ("[a]", "a"),
    ("[abcd]", "a"),
)]
#[benches::valid_ranges(
    ("[a-z]", "a"),
    ("[a-zA-Z]", "a"),
)]
#[benches::mixed_valid(
    ("[a-zABC]", "a"),
    ("[a-zABCD-Z]", "a"),
)]
fn bench_dowild_with_character_classes(pattern: &str, haystack: &str) -> bool {
    black_box(dowild_with(
        black_box(pattern.as_bytes()),
        black_box(haystack.as_bytes()),
        black_box(OPTIONS_WITH_CLASSES),
    ))
}

library_benchmark_group!(
    name = dowild;
    benchmarks =
        bench_dowild_with_character_classes,
        bench_dowild_with_wildcard_any,
        bench_dowild_without_wildcards
);
main!(library_benchmark_groups = dowild);
