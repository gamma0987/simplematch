use clap::{Parser, Subcommand};
use common::{pattern_to_regex, PatternOptions};
use rand::distr::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use serde::{Deserialize, Serialize};

#[derive(Subcommand)]
pub enum Commands {
    /// Generate the random data for the benchmarks. Output is one benchmark as json per line
    GenerateRandom {
        #[arg(short = 'k', long)]
        pattern_length: usize,
        #[arg(short = 'l', long)]
        haystack_length: usize,
        #[arg(short = 'n', long)]
        amount: usize,
        #[arg(short = 'a', long, default_value = "true")]
        wildcard_any: bool,
        #[arg(short = 'o', long, default_value = "false")]
        wildcard_one: bool,
        #[arg(short = 'p', long)]
        pattern: String,
        #[arg(short = 'y', long)]
        haystack: String,
    },
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Benchmark {
    pub pattern: String,
    pub haystack: String,
    pub is_match: bool,
}

#[derive(Debug)]
pub struct Generator(ThreadRng);

impl Generator {
    pub fn new() -> Self {
        Self(rand::rng())
    }
    pub fn generate_pattern(
        &mut self,
        length: usize,
        wildcard_any: bool,
        wildcard_one: bool,
        allowed_chars: &str,
    ) -> String {
        RandomString::new(allowed_chars.to_owned(), wildcard_any, wildcard_one)
            .sample_iter(&mut self.0)
            .take(length)
            .collect::<String>()
    }

    pub fn generate_haystack(&mut self, length: usize, allowed_chars: &str) -> String {
        RandomString::new(allowed_chars.to_owned(), false, false)
            .sample_iter(&mut self.0)
            .take(length)
            .collect::<String>()
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}

struct RandomString {
    allowed_chars: String,
}

impl RandomString {
    fn new(mut allowed_chars: String, wildcard_any: bool, wildcard_one: bool) -> Self {
        if wildcard_one {
            allowed_chars.push('?');
        }
        if wildcard_any {
            allowed_chars.push('*');
        }
        Self { allowed_chars }
    }
}

impl Distribution<char> for RandomString {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> char {
        let uniform = Uniform::new(0, self.allowed_chars.len())
            .expect("The uniform distribution should be valid");
        self.allowed_chars
            .chars()
            .nth(uniform.sample(rng))
            .expect("The index should be valid")
    }
}

pub fn generate_random(commands: Commands) {
    let Commands::GenerateRandom {
        pattern_length,
        haystack_length,
        amount,
        wildcard_any,
        wildcard_one,
        pattern,
        haystack,
    } = commands;

    let mut generator = Generator::default();
    let mut generated = 0;
    let mut num_is_match = 0;
    let mut num_not_match = 0;
    let max_num_retries = 500;
    let mut num_retries = 0;
    while generated < amount {
        let pattern =
            generator.generate_pattern(pattern_length, wildcard_any, wildcard_one, &pattern);

        if wildcard_any && !pattern.contains('*') {
            continue;
        }
        if wildcard_one && !pattern.contains('?') {
            continue;
        }

        let regex = pattern_to_regex(&pattern, PatternOptions::default())
            .expect("The pattern should be a valid regex");

        let haystack = generator.generate_haystack(haystack_length, &haystack);
        let is_match = regex.is_match(haystack.as_bytes());
        // Generate an equal amount of non-matching and matching patterns
        if is_match {
            if num_is_match > amount / 2 && num_retries < max_num_retries {
                num_retries += 1;
                continue;
            }
            num_retries = 0;
            num_is_match += 1;
        } else {
            if num_not_match > amount / 2 && num_retries < max_num_retries {
                num_retries += 1;
                continue;
            }
            num_retries = 0;
            num_not_match += 1;
        }

        let benchmark = Benchmark {
            pattern,
            haystack,
            is_match,
        };

        let data = serde_json::to_string(&benchmark).expect("A benchmark should be valid json");
        println!("{data}");

        generated += 1;
    }
}
