<!-- spell-checker: ignore fixt binstall libtest eprintln usize Gjengset println combinators -->
<!-- spell-checker: ignore fooa -->
<!-- markdownlint-disable MD041 MD033 -->

<h1 align="center"><code>simplematch</code></h1>

<div align="center">
    <a href="https://docs.rs/crate/simplematch/">Released API Docs</a>
    |
    <a href="https://github.com/gamma0987/simplematch/blob/main/CHANGELOG.md">Changelog</a>
</div>
<br>
<div align="center">
    <a href="https://github.com/gamma0987/simplematch/actions/workflows/cicd.yml">
        <img
        src="https://github.com/gamma0987/simplematch/actions/workflows/cicd.yml/badge.svg"
        alt="GitHub branch checks state"/>
    </a>
    <a href="https://codecov.io/gh/gamma0987/simplematch" >
         <img
         src="https://codecov.io/gh/gamma0987/simplematch/graph/badge.svg?token=GHG1BMO029"
         alt="Coverage"/>
     </a>
    <a href="https://crates.io/crates/simplematch">
        <img src="https://img.shields.io/crates/v/simplematch.svg" alt="Crates.io"/>
    </a>
    <a href="https://docs.rs/simplematch/">
        <img src="https://docs.rs/simplematch/badge.svg" alt="docs.rs"/>
    </a>
</div>
<hr>

# simplematch

`simplematch` is a Rust library that provides fast extended wildcard pattern
matching for strings and bytes with a simple and intuitive API.

## Features

Supports the basic wildcards `*` (matches any sequence of characters), `?`
(matches a single character). Optionally enable escaping `\` of special
characters or enable character classes `[...]`. Character classes can be negated
`[!...]` and contain ranges `[a-zA-Z]`.

* Optimized for **performance**
* **Intuitive** and **simple** API consisting of two functions `dowild` and
  `dowild_with` with custom pattern matching `Options`
* **Customizable** wildcard characters and matching options like
  `case-insensitive`
* `#![no_std]` compatible (when the `std` feature is disabled)
* Fully **documented** on [docs.rs](https://docs.rs/simplematch)

## Examples

The basic function `dowild`:

```rust
use simplematch::dowild;

assert_eq!(dowild("foo*".as_bytes(), "foobar".as_bytes()), true);
assert_eq!(dowild("foo?".as_bytes(), "fooa".as_bytes()), true)
```

or more conveniently, bring the `DoWild` trait in scope to match directly
on strings (and bytes) without performance loss:

```rust
use simplematch::DoWild;

assert_eq!("foobar".dowild("foo*"), true);
```

Use `dowild_with` with `Options` to customize the pattern matching:

```rust
use simplematch::{dowild_with, Options, DoWild};

let options = Options::default()
    .case_insensitive(true)
    .wildcard_any_with(b'%');

assert_eq!(
    "FOObar".dowild_with("foo%", options),
    true
);
```

## Installation

Add `simplematch` to your `Cargo.toml`:

```toml
[dependencies]
simplematch = "0.2.0"
```

Or use [`cargo add`](https://github.com/killercup/cargo-edit):

```bash
cargo add simplematch@0.2.0
```

## Benchmarks

The benchmarks below show the average instruction counts of each function for a
given pattern and haystack. The haystacks and patterns are random valid utf-8
strings each with variable length.

| library/haystack length<br>(samples)           | `128`<br>`(100)` | `512`<br>`(100)` | `1000`<br>`(100)` | `10000`<br>`(100)` | `50000`<br>`(100)` | `100000`<br>`(100)` |
| :--------------------------------------------- | ---------------: | ---------------: | ----------------: | -----------------: | -----------------: | ------------------: |
| simplematch::dowild                            |           `1748` |           `5068` |            `8227` |            `88000` |           `420132` |            `781949` |
| simplematch::dowild_with                       |           `2440` |           `7426` |           `11951` |           `130723` |           `635673` |           `1173179` |
| regex::bytes::Regex::is_match<br>(precompiled) |         `202749` |         `258994` |          `271740` |           `411166` |           `748247` |           `1066587` |
| wildcard::Wildcard::is_match                   |           `2910` |           `6347` |           `13313` |           `134660` |           `530098` |           `1053973` |
| wildmatch::Wildmatch::matches                  |           `4925` |          `13016` |           `22968` |           `232897` |          `1105721` |           `2122124` |

To be able to run these benchmarks, you need
[`iai-callgrind`](https://crates.io/crates/iai-callgrind) installed. Then run
the benchmarks from above with `cargo bench -p benchmarks --bench random`.

## License

`simplematch` is dual licensed under the Apache 2.0 license and the MIT license
at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as in
[License](#license), without any additional terms or conditions.
