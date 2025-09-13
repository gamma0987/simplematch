use clap::{Parser, Subcommand};
use common::{pattern_to_regex, PatternOptions};
use rand::distr::uniform::{UniformSampler, UniformUsize};
use rand::distr::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Subcommand)]
pub enum Commands {
    /// Generate the random data for the benchmarks. Output is one benchmark as json per line
    GenerateRandomBenchmark {
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
    ImportIaiCallgrindSchema,
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

    pub fn try_generate_non_matching_haystack(
        &mut self,
        pattern: &str,
        length: usize,
        allowed_chars: &str,
    ) -> String {
        let matching_haystack = self.generate_matching_haystack(pattern, length, allowed_chars);

        let mut random_indices: Vec<usize> = (0..matching_haystack.len()).collect();
        random_indices.shuffle(&mut self.0);
        let (random_indices, _) = random_indices.split_at(matching_haystack.len() / 5);

        let mut chars = matching_haystack.chars().collect::<Vec<char>>();

        let sampler = RandomString::new(allowed_chars.to_owned(), false, false);
        for index in random_indices {
            chars[*index] = sampler.sample(&mut self.0)
        }

        chars.iter().collect()
    }

    pub fn generate_matching_haystack(
        &mut self,
        pattern: &str,
        length: usize,
        allowed_chars: &str,
    ) -> String {
        assert!(pattern.chars().count() <= length);

        let is_wildcard_any = |c| c == '*';
        let replace_wildcard_one = |s: &str| -> String {
            s.chars()
                .map(|c| {
                    if c == '?' {
                        RandomString::new(allowed_chars.to_owned(), false, false).sample(&mut self.0)
                    } else {
                        c
                    }
                })
                .collect()
        };

        let (mut num_chars, mut splits) = pattern
            .split(is_wildcard_any)
            .map(replace_wildcard_one)
            .fold((0, vec![]), |(num_chars, mut acc), elem| {
                let elem_length = elem.chars().count();
                acc.push(elem);

                (num_chars + elem_length, acc)
            });

        let mut random_indices: Vec<usize> = (0..splits.len()).collect();
        random_indices.shuffle(&mut self.0);

        for random_index in random_indices {
            if random_index != splits.len() - 1 {
                let high = length - num_chars;
                let random_length = if high > 0 {
                    UniformUsize::new_inclusive(0, high)
                        .unwrap()
                        .sample(&mut self.0)
                } else {
                    break;
                };

                num_chars += random_length;

                let split = splits.get_mut(random_index).unwrap();
                let random_data: String = RandomString::new(allowed_chars.to_owned(), false, false)
                    .sample_iter(&mut self.0)
                    .take(random_length)
                    .collect();

                split.push_str(&random_data);
            }
        }

        if num_chars < length {
            let first = splits.first_mut().unwrap();
            let random_data: String = RandomString::new(allowed_chars.to_owned(), false, false)
                .sample_iter(&mut self.0)
                .take(length - num_chars)
                .collect();
            first.push_str(&random_data);
        }

        splits.into_iter().collect()
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

pub fn generate_random(
    pattern_length: usize,
    haystack_length: usize,
    amount: usize,
    wildcard_any: bool,
    wildcard_one: bool,
    pattern: String,
    haystack: String,
) {
    let mut generator = Generator::default();
    let mut generated = 0;
    let mut num_is_match = 0;
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

        let (haystack, is_match) = loop {
            if num_is_match <= amount / 2 {
                let haystack =
                    generator.generate_matching_haystack(&pattern, haystack_length, &haystack);
                if regex.is_match(haystack.as_bytes()) {
                    num_is_match += 1;
                    break (haystack, true);
                } else {
                    continue;
                }
            } else {
                let haystack = generator.try_generate_non_matching_haystack(
                    &pattern,
                    haystack_length,
                    &haystack,
                );
                if regex.is_match(haystack.as_bytes()) {
                    continue;
                } else {
                    break (haystack, false);
                }
            }
        };

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
