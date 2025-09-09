use clap::Parser;
use xtask::{generate_random, Commands};

fn main() {
    let args = xtask::Args::parse();
    if let Some(c @ Commands::GenerateRandom { .. }) = args.command {
        generate_random(c);
    }
}
