use std::process::Command;

use clap::Parser;
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
        }
    }
}
