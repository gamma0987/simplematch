use std::hint::black_box;

use iai_callgrind::{
    library_benchmark, library_benchmark_group, main, Callgrind, LibraryBenchmarkConfig,
};
use serde::{Deserialize, Serialize};
use simplematch::{dowild, dowild_with, Options};
use wildcard::Wildcard;

#[derive(Serialize, Deserialize)]
struct JsonFixture {
    pattern: String,
    haystack: String,
    is_match: bool,
}

fn setup_json(input: String) -> (String, String, bool) {
    let json: JsonFixture = serde_json::from_str(&input).unwrap();
    (json.pattern, json.haystack, json.is_match)
}

fn verify_match((actual, expected): (bool, bool)) {
    assert_eq!(actual, expected);
}

#[library_benchmark]
#[benches::pattern_5_haystack_128(
    file = "benchmarks/fixtures/pattern_5_haystack_128.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_5_haystack_512(
    file = "benchmarks/fixtures/pattern_5_haystack_512.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_5_haystack_1000(
    file = "benchmarks/fixtures/pattern_5_haystack_1000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_5_haystack_10000(
    file = "benchmarks/fixtures/pattern_5_haystack_10000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_5_haystack_50000(
    file = "benchmarks/fixtures/pattern_5_haystack_50000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_5_haystack_100000(
    file = "benchmarks/fixtures/pattern_5_haystack_100000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_10_haystack_128(
    file = "benchmarks/fixtures/pattern_10_haystack_128.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_10_haystack_512(
    file = "benchmarks/fixtures/pattern_10_haystack_512.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_10_haystack_1000(
    file = "benchmarks/fixtures/pattern_10_haystack_1000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_10_haystack_10000(
    file = "benchmarks/fixtures/pattern_10_haystack_10000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_10_haystack_50000(
    file = "benchmarks/fixtures/pattern_10_haystack_50000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_10_haystack_100000(
    file = "benchmarks/fixtures/pattern_10_haystack_100000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_20_haystack_128(
    file = "benchmarks/fixtures/pattern_20_haystack_128.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_20_haystack_512(
    file = "benchmarks/fixtures/pattern_20_haystack_512.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_20_haystack_1000(
    file = "benchmarks/fixtures/pattern_20_haystack_1000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_20_haystack_10000(
    file = "benchmarks/fixtures/pattern_20_haystack_10000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_20_haystack_50000(
    file = "benchmarks/fixtures/pattern_20_haystack_50000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_20_haystack_100000(
    file = "benchmarks/fixtures/pattern_20_haystack_100000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_40_haystack_128(
    file = "benchmarks/fixtures/pattern_40_haystack_128.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_40_haystack_512(
    file = "benchmarks/fixtures/pattern_40_haystack_512.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_40_haystack_1000(
    file = "benchmarks/fixtures/pattern_40_haystack_1000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_40_haystack_10000(
    file = "benchmarks/fixtures/pattern_40_haystack_10000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_40_haystack_50000(
    file = "benchmarks/fixtures/pattern_40_haystack_50000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_40_haystack_100000(
    file = "benchmarks/fixtures/pattern_40_haystack_100000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_80_haystack_128(
    file = "benchmarks/fixtures/pattern_80_haystack_128.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_80_haystack_512(
    file = "benchmarks/fixtures/pattern_80_haystack_512.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_80_haystack_1000(
    file = "benchmarks/fixtures/pattern_80_haystack_1000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_80_haystack_10000(
    file = "benchmarks/fixtures/pattern_80_haystack_10000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_80_haystack_50000(
    file = "benchmarks/fixtures/pattern_80_haystack_50000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_80_haystack_100000(
    file = "benchmarks/fixtures/pattern_80_haystack_100000.json",
    setup = setup_json,
    teardown = verify_match
)]
fn bench_simplematch((pattern, haystack, is_match): (String, String, bool)) -> (bool, bool) {
    (
        black_box(dowild(
            black_box(pattern.as_bytes()),
            black_box(haystack.as_bytes()),
        )),
        is_match,
    )
}

#[library_benchmark]
#[benches::pattern_5_haystack_128(
    file = "benchmarks/fixtures/pattern_5_haystack_128.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_5_haystack_512(
    file = "benchmarks/fixtures/pattern_5_haystack_512.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_5_haystack_1000(
    file = "benchmarks/fixtures/pattern_5_haystack_1000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_5_haystack_10000(
    file = "benchmarks/fixtures/pattern_5_haystack_10000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_5_haystack_50000(
    file = "benchmarks/fixtures/pattern_5_haystack_50000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_5_haystack_100000(
    file = "benchmarks/fixtures/pattern_5_haystack_100000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_10_haystack_128(
    file = "benchmarks/fixtures/pattern_10_haystack_128.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_10_haystack_512(
    file = "benchmarks/fixtures/pattern_10_haystack_512.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_10_haystack_1000(
    file = "benchmarks/fixtures/pattern_10_haystack_1000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_10_haystack_10000(
    file = "benchmarks/fixtures/pattern_10_haystack_10000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_10_haystack_50000(
    file = "benchmarks/fixtures/pattern_10_haystack_50000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_10_haystack_100000(
    file = "benchmarks/fixtures/pattern_10_haystack_100000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_20_haystack_128(
    file = "benchmarks/fixtures/pattern_20_haystack_128.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_20_haystack_512(
    file = "benchmarks/fixtures/pattern_20_haystack_512.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_20_haystack_1000(
    file = "benchmarks/fixtures/pattern_20_haystack_1000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_20_haystack_10000(
    file = "benchmarks/fixtures/pattern_20_haystack_10000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_20_haystack_50000(
    file = "benchmarks/fixtures/pattern_20_haystack_50000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_20_haystack_100000(
    file = "benchmarks/fixtures/pattern_20_haystack_100000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_40_haystack_128(
    file = "benchmarks/fixtures/pattern_40_haystack_128.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_40_haystack_512(
    file = "benchmarks/fixtures/pattern_40_haystack_512.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_40_haystack_1000(
    file = "benchmarks/fixtures/pattern_40_haystack_1000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_40_haystack_10000(
    file = "benchmarks/fixtures/pattern_40_haystack_10000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_40_haystack_50000(
    file = "benchmarks/fixtures/pattern_40_haystack_50000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_40_haystack_100000(
    file = "benchmarks/fixtures/pattern_40_haystack_100000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_80_haystack_128(
    file = "benchmarks/fixtures/pattern_80_haystack_128.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_80_haystack_512(
    file = "benchmarks/fixtures/pattern_80_haystack_512.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_80_haystack_1000(
    file = "benchmarks/fixtures/pattern_80_haystack_1000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_80_haystack_10000(
    file = "benchmarks/fixtures/pattern_80_haystack_10000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_80_haystack_50000(
    file = "benchmarks/fixtures/pattern_80_haystack_50000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_80_haystack_100000(
    file = "benchmarks/fixtures/pattern_80_haystack_100000.json",
    setup = setup_json,
    teardown = verify_match
)]
fn bench_simplematch_dowild_with(
    (pattern, haystack, is_match): (String, String, bool),
) -> (bool, bool) {
    (
        black_box(dowild_with(
            black_box(pattern.as_bytes()),
            black_box(haystack.as_bytes()),
            black_box(Options::default()),
        )),
        is_match,
    )
}

#[library_benchmark]
#[benches::pattern_5_haystack_128(
    file = "benchmarks/fixtures/pattern_5_haystack_128.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_5_haystack_512(
    file = "benchmarks/fixtures/pattern_5_haystack_512.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_5_haystack_1000(
    file = "benchmarks/fixtures/pattern_5_haystack_1000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_5_haystack_10000(
    file = "benchmarks/fixtures/pattern_5_haystack_10000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_5_haystack_50000(
    file = "benchmarks/fixtures/pattern_5_haystack_50000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_5_haystack_100000(
    file = "benchmarks/fixtures/pattern_5_haystack_100000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_10_haystack_128(
    file = "benchmarks/fixtures/pattern_10_haystack_128.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_10_haystack_512(
    file = "benchmarks/fixtures/pattern_10_haystack_512.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_10_haystack_1000(
    file = "benchmarks/fixtures/pattern_10_haystack_1000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_10_haystack_10000(
    file = "benchmarks/fixtures/pattern_10_haystack_10000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_10_haystack_50000(
    file = "benchmarks/fixtures/pattern_10_haystack_50000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_10_haystack_100000(
    file = "benchmarks/fixtures/pattern_10_haystack_100000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_20_haystack_128(
    file = "benchmarks/fixtures/pattern_20_haystack_128.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_20_haystack_512(
    file = "benchmarks/fixtures/pattern_20_haystack_512.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_20_haystack_1000(
    file = "benchmarks/fixtures/pattern_20_haystack_1000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_20_haystack_10000(
    file = "benchmarks/fixtures/pattern_20_haystack_10000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_20_haystack_50000(
    file = "benchmarks/fixtures/pattern_20_haystack_50000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_20_haystack_100000(
    file = "benchmarks/fixtures/pattern_20_haystack_100000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_40_haystack_128(
    file = "benchmarks/fixtures/pattern_40_haystack_128.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_40_haystack_512(
    file = "benchmarks/fixtures/pattern_40_haystack_512.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_40_haystack_1000(
    file = "benchmarks/fixtures/pattern_40_haystack_1000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_40_haystack_10000(
    file = "benchmarks/fixtures/pattern_40_haystack_10000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_40_haystack_50000(
    file = "benchmarks/fixtures/pattern_40_haystack_50000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_40_haystack_100000(
    file = "benchmarks/fixtures/pattern_40_haystack_100000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_80_haystack_128(
    file = "benchmarks/fixtures/pattern_80_haystack_128.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_80_haystack_512(
    file = "benchmarks/fixtures/pattern_80_haystack_512.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_80_haystack_1000(
    file = "benchmarks/fixtures/pattern_80_haystack_1000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_80_haystack_10000(
    file = "benchmarks/fixtures/pattern_80_haystack_10000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_80_haystack_50000(
    file = "benchmarks/fixtures/pattern_80_haystack_50000.json",
    setup = setup_json,
    teardown = verify_match
)]
#[benches::pattern_80_haystack_100000(
    file = "benchmarks/fixtures/pattern_80_haystack_100000.json",
    setup = setup_json,
    teardown = verify_match
)]
fn bench_wildcard((pattern, haystack, is_match): (String, String, bool)) -> (bool, bool) {
    (
        black_box(
            black_box(black_box(Wildcard::new(black_box(pattern.as_bytes()))).unwrap())
                .is_match(black_box(haystack.as_bytes())),
        ),
        is_match,
    )
}

library_benchmark_group!(
    name = random;
    compare_by_id = true;
    benchmarks = bench_simplematch, bench_simplematch_dowild_with, bench_wildcard
);

main!(
    config = LibraryBenchmarkConfig::default()
        .tool(Callgrind::default()
            .args(["--dump-instr=yes"])
        );
    library_benchmark_groups = random
);
