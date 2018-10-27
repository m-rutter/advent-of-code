use failure::Error;

use itertools::Itertools;
use std::process;

use super::{AoCSolution, Config};

pub fn run(config: &Config) -> AoCSolution {
    let input = parse_input(&config.input).unwrap_or_else(|err| {
        eprintln!("Problem parsing input: {}", err);
        process::exit(1);
    });

    let checksum = gen_checksum(&input);
    let sum_of_even_divisons = users_are_odd(&input);

    AoCSolution {
        part_one: checksum.to_string(),
        part_two: sum_of_even_divisons.to_string(),
    }
}

fn gen_checksum(v: &[Vec<u32>]) -> u32 {
    v.iter()
        .map(|row| {
            let max = match row.iter().max() {
                Some(&num) => num,
                None => 0,
            };

            let min = match row.iter().min() {
                Some(&num) => num,
                None => 0,
            };

            max - min
        })
        .sum()
}

fn users_are_odd(v: &[Vec<u32>]) -> u32 {
    v.iter()
        .map(|inner_v| {
            inner_v
                .iter()
                .tuple_combinations()
                .map(|(a, b)| {
                    let sum = if a % b == 0 { a / b } else { 0 };

                    if b % a == 0 {
                        b / a + sum
                    } else {
                        sum
                    }
                })
                .sum::<u32>()
        })
        .sum()
}

fn parse_input(input: &str) -> Result<Vec<Vec<u32>>, Error> {
    let table: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|elem| elem.parse().ok())
                .collect()
        })
        .collect();

    Ok(table)
}
