use core::fmt::Write;
use std::collections::BTreeMap;
use std::fs::File;
use std::process::Command;

use clap::Parser;
use xtask::iai_callgrind::{BenchmarkSummary, EitherOrBoth2, Metric, ToolMetricSummary};
use xtask::{generate_random, Commands};

fn main() {
    let args = xtask::Args::parse();
    if let Some(commands) = args.command {
        match commands {
            Commands::GenerateRandomBenchmark {
                pattern_length,
                haystack_length,
                amount,
                wildcard_any,
                wildcard_one,
                pattern,
                haystack,
            } => generate_random(
                pattern_length,
                haystack_length,
                amount,
                wildcard_any,
                wildcard_one,
                pattern,
                haystack,
            ),
            Commands::ImportIaiCallgrindSchema => {
                let current_dir =
                    std::env::current_dir().expect("The current directory should be valid");
                let status = Command::new("cargo")
                    .args(["typify", "-o"])
                    .arg(
                        current_dir
                            .join("xtask")
                            .join("src")
                            .join("iai_callgrind.rs"),
                    )
                    .arg(current_dir.join("xtask").join("iai-callgrind.schema.json"))
                    .status()
                    .expect("Running cargo-typify should succeed");
                if !status.success() {
                    std::process::exit(1)
                }
            }
            Commands::SummarizeRandomBenchmarks => {
                let current_dir =
                    std::env::current_dir().expect("The current directory should be valid");
                let benchmarks_dir = current_dir
                    .join("target")
                    .join("iai")
                    .join("benchmarks")
                    .join("random")
                    .join("random");

                let mut simplematch_dowild_sum = 0;
                let mut simplematch_dowild_stats: BTreeMap<usize, (usize, u64)> = BTreeMap::new();

                let mut simplematch_dowild_with_sum = 0;
                let mut simplematch_dowild_with_stats: BTreeMap<usize, (usize, u64)> =
                    BTreeMap::new();

                let mut wildcard_sum = 0;
                let mut wildcard_stats: BTreeMap<usize, (usize, u64)> = BTreeMap::new();

                let mut wildmatch_sum = 0;
                let mut wildmatch_stats: BTreeMap<usize, (usize, u64)> = BTreeMap::new();

                let mut regex_sum = 0;
                let mut regex_stats: BTreeMap<usize, (usize, u64)> = BTreeMap::new();

                for entry in std::fs::read_dir(benchmarks_dir)
                    .unwrap()
                    .map(Result::unwrap)
                {
                    let path = entry.path();
                    let summary_path = path.join("summary.json");
                    let summary: BenchmarkSummary = serde_json::from_reader(
                        File::open(summary_path).expect("The summary path should be valid"),
                    )
                    .expect("The summary should be valid json");

                    for profile in summary.profiles.0 {
                        let total = profile.summaries.total;
                        // dbg!(&summary.id);
                        if let [_, split, _] = summary
                            .id
                            .as_ref()
                            .expect("An id should be present")
                            .rsplitn(3, "_")
                            .collect::<Vec<&str>>()[..]
                        {
                            let haystack_length: usize = split
                                .parse()
                                .expect("The haystack length should be a valid");

                            if let ToolMetricSummary::Callgrind(metrics_summary) = total.summary {
                                let ir = metrics_summary.0.get("Ir").unwrap();
                                match &ir.metrics {
                                    EitherOrBoth2::Both(metric, _) | EitherOrBoth2::Left(metric) => {
                                        match metric {
                                            Metric::Int(value)
                                                if summary.function_name
                                                    == "bench_simplematch_dowild" =>
                                            {
                                                simplematch_dowild_sum += value;

                                                simplematch_dowild_stats
                                                    .entry(haystack_length)
                                                    .and_modify(|(n, v)| {
                                                        *n += 1;
                                                        *v += value;
                                                    })
                                                    .or_insert((1, *value));
                                            }
                                            Metric::Int(value)
                                                if summary.function_name
                                                    == "bench_simplematch_dowild_with" =>
                                            {
                                                simplematch_dowild_with_sum += value;

                                                simplematch_dowild_with_stats
                                                    .entry(haystack_length)
                                                    .and_modify(|(n, v)| {
                                                        *n += 1;
                                                        *v += value;
                                                    })
                                                    .or_insert((1, *value));
                                            }
                                            Metric::Int(value)
                                                if summary.function_name == "bench_wildcard" =>
                                            {
                                                wildcard_sum += value;

                                                wildcard_stats
                                                    .entry(haystack_length)
                                                    .and_modify(|(n, v)| {
                                                        *n += 1;
                                                        *v += value;
                                                    })
                                                    .or_insert((1, *value));
                                            }
                                            Metric::Int(value)
                                                if summary.function_name == "bench_wildmatch" =>
                                            {
                                                wildmatch_sum += value;

                                                wildmatch_stats
                                                    .entry(haystack_length)
                                                    .and_modify(|(n, v)| {
                                                        *n += 1;
                                                        *v += value;
                                                    })
                                                    .or_insert((1, *value));
                                            }
                                            Metric::Int(value)
                                                if summary.function_name == "bench_regex" =>
                                            {
                                                regex_sum += value;

                                                regex_stats
                                                    .entry(haystack_length)
                                                    .and_modify(|(n, v)| {
                                                        *n += 1;
                                                        *v += value;
                                                    })
                                                    .or_insert((1, *value));
                                            }
                                            Metric::Float(_) | Metric::Int(_) => continue,
                                        }
                                    }
                                    EitherOrBoth2::Right(_) => continue,
                                }
                            }
                        }
                    }
                }

                let mut headline = "| library/haystack length<br>(samples) |".to_string();
                let mut separator = "|: --- |".to_string();
                let mut table = String::new();

                let mut line = "simplematch::dowild".to_string();
                for (length, (num, sum)) in simplematch_dowild_stats {
                    write!(headline, "`{length}`<br>`({num})` |").unwrap();
                    write!(separator, " ---:|").unwrap();

                    write!(line, "| `{}`", sum / (num as u64)).unwrap();
                }

                writeln!(table, "{headline}").unwrap();
                writeln!(table, "{separator}").unwrap();
                writeln!(table, "{line}").unwrap();

                line = "simplematch::dowild_with".to_string();
                for (_, (num, sum)) in simplematch_dowild_with_stats {
                    write!(line, "| `{}`", sum / (num as u64)).unwrap();
                }
                writeln!(table, "{line}").unwrap();

                line = "regex::bytes::Regex::is_match<br>(precompiled)".to_string();
                for (_, (num, sum)) in regex_stats {
                    write!(line, "| `{}`", (sum / num as u64)).unwrap();
                }
                writeln!(table, "{line}").unwrap();

                line = "wildcard::Wildcard::is_match".to_string();
                for (_, (num, sum)) in wildcard_stats {
                    write!(line, "| `{}`", (sum / num as u64)).unwrap();
                }
                writeln!(table, "{line}").unwrap();

                line = "wildmatch::Wildmatch::matches".to_string();
                for (_, (num, sum)) in wildmatch_stats {
                    write!(line, "| `{}`", (sum / num as u64)).unwrap();
                }
                writeln!(table, "{line}").unwrap();

                let final_table = mtf::process(&table).unwrap();
                println!("{}", final_table.trim());
            }
        }
    }
}
