// spell-checker: ignore dolore

use std::hint::black_box;

use iai_callgrind::{
    library_benchmark, library_benchmark_group, main, Dhat, LibraryBenchmarkConfig,
};
use simplematch::{dowild, dowild_with, Options};
use wildcard::Wildcard;
use wildmatch::WildMatch;

// spell-checker: disable
const HAYSTACK: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const LOREM_PARAGRAPH: &str =
    "Vestibulum dignissim vestibulum purus, ac euismod diam lacinia et. Proin pretium condimentum \
     ligula at consectetur. Ut pellentesque ligula vel vestibulum ornare. Vivamus pellentesque \
     condimentum hendrerit. Mauris nunc mauris, tristique eu mattis vel, pellentesque nec arcu. \
     Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; \
     Curabitur cursus at eros nec posuere. Aenean facilisis in risus sed hendrerit. Nullam \
     tincidunt lectus leo, a sodales quam semper vitae. Morbi eu varius orci. Phasellus sit amet \
     felis nec nisl placerat accumsan non vitae dui. Cras ac ipsum ut tellus consectetur \
     ullamcorper sed ac tellus. Ut tincidunt eleifend arcu, id porttitor quam ullamcorper quis.";
const LOREM_PATTERN: &str = "Ve*ve*Aenean??acc*ac****ipsum?ut*?ur ???????????*ac*ulla****?*.";
const DEFAULT_OPTIONS: Options<u8> = Options::new();
// spell-checker: enable

#[library_benchmark]
#[benches::a_star("a*b", "a*a*b", "a*a*a*b")]
#[bench::a_max_star(format!("{}b", "a*".repeat(50)).as_str())]
#[bench::a_stars(format!("a{}b", "*".repeat(100)).as_str())]
fn bench_simplematch(pattern: &str) -> bool {
    black_box(dowild(
        black_box(pattern.as_bytes()),
        black_box(HAYSTACK.as_bytes()),
    ))
}

#[library_benchmark]
#[bench::lorem(LOREM_PATTERN, LOREM_PARAGRAPH)]
fn bench_simplematch_lorem(pattern: &str, input: &str) -> bool {
    black_box(dowild(black_box(pattern.as_bytes()), input.as_bytes()))
}

#[library_benchmark]
#[benches::a_star("a*b", "a*a*b", "a*a*a*b")]
#[bench::a_max_star(format!("{}b", "a*".repeat(50)).as_str())]
#[bench::a_stars(format!("a{}b", "*".repeat(100)).as_str())]
fn bench_simplematch_dowild_with_default(pattern: &str) -> bool {
    black_box(dowild_with(
        black_box(pattern.as_bytes()),
        black_box(HAYSTACK.as_bytes()),
        black_box(DEFAULT_OPTIONS),
    ))
}

#[library_benchmark]
#[bench::lorem(LOREM_PATTERN, LOREM_PARAGRAPH)]
fn bench_simplematch_dowild_with_default_lorem(pattern: &str, input: &str) -> bool {
    black_box(dowild_with(
        black_box(pattern.as_bytes()),
        black_box(input.as_bytes()),
        black_box(DEFAULT_OPTIONS),
    ))
}

#[library_benchmark]
#[benches::a_star("a*b", "a*a*b", "a*a*a*b")]
#[bench::a_max_star(format!("{}b", "a*".repeat(50)).as_str())]
#[bench::a_stars(format!("a{}b", "*".repeat(100)).as_str())]
fn bench_wildcard(pattern: &str) -> bool {
    black_box(
        black_box(black_box(Wildcard::new(black_box(pattern.as_bytes()))).unwrap())
            .is_match(black_box(HAYSTACK.as_bytes())),
    )
}

#[library_benchmark]
#[bench::lorem(LOREM_PATTERN, LOREM_PARAGRAPH)]
fn bench_wildcard_lorem(pattern: &str, input: &str) -> bool {
    black_box(
        black_box(black_box(Wildcard::new(black_box(pattern.as_bytes()))).unwrap())
            .is_match(input.as_bytes()),
    )
}

#[library_benchmark(
    config = LibraryBenchmarkConfig::default().tool(Dhat::default())
)]
#[benches::a_star("a*b", "a*a*b", "a*a*a*b")]
#[bench::a_max_star(format!("{}b", "a*".repeat(50)).as_str())]
#[bench::a_stars(format!("a{}b", "*".repeat(100)).as_str())]
fn bench_wildmatch(pattern: &str) -> bool {
    black_box(black_box(WildMatch::new(black_box(pattern))).matches(black_box(HAYSTACK)))
}

#[library_benchmark(
    config = LibraryBenchmarkConfig::default().tool(Dhat::default())
)]
#[bench::lorem(LOREM_PATTERN, LOREM_PARAGRAPH)]
fn bench_wildmatch_lorem(pattern: &str, input: &str) -> bool {
    black_box(black_box(WildMatch::new(black_box(pattern))).matches(input))
}

library_benchmark_group!(
    name = just_a;
    compare_by_id = true;
    benchmarks =
        bench_simplematch,
        bench_simplematch_dowild_with_default,
        bench_wildcard,
        bench_wildmatch
);

library_benchmark_group!(
    name = lorem;
    compare_by_id = true;
    benchmarks =
        bench_simplematch_lorem,
        bench_simplematch_dowild_with_default_lorem,
        bench_wildcard_lorem,
        bench_wildmatch_lorem
);

main!(library_benchmark_groups = just_a, lorem);
