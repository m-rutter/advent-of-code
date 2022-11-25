use itertools::Itertools;

use crate::{error, Solution};

/// Compute the solution to day 2 of AoC 2017
pub fn run(input: &str) -> error::Result<Solution> {
    let input = parse_input(&input);

    if input.is_empty() {
        Err(error::ParsingError::ParseError)?;
    }

    let checksum = gen_checksum(&input);
    let sum_of_even_divisons = users_are_odd(&input);

    Ok(Solution {
        part_one: checksum.to_string(),
        part_two: sum_of_even_divisons.to_string(),
    })
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

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .filter_map(|line| {
            let record: Vec<u32> = line
                .split_whitespace()
                .filter_map(|elem| elem.parse().ok())
                .collect();

            if record.is_empty() {
                None
            } else {
                Some(record)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_offical_result() {
        let input = include_str!("./input/day02");

        let result = run(&input).unwrap();

        assert_eq!(result.part_one, "36766");
        assert_eq!(result.part_two, "261");
    }
}
