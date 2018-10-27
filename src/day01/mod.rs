use failure::Error;
use std::process;

use super::{AoCSolution, Config};

pub fn run(config: &Config) -> AoCSolution {
    let input = parser(&config.input).unwrap_or_else(|err| {
        eprintln!("Problem parsing input: {}", err);
        process::exit(1);
    });

    let part_one = circular_match_and_sum(&input, 1);
    let part_two = circular_match_and_sum(&input, input.len() / 2);

    AoCSolution {
        part_one: part_one.to_string(),
        part_two: part_two.to_string(),
    }
}

fn parser(input: &str) -> Result<Vec<u32>, Error> {
    let v: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();

    Ok(v)
}

fn circular_match_and_sum(v: &[u32], offset: usize) -> u32 {
    v.iter()
        .cycle()
        .skip(offset)
        .zip(v.iter())
        .filter_map(|(a, b)| if a == b { Some(a) } else { None }) // if blocks are expressions? awesome!
        .sum()
}
