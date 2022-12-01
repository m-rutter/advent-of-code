use itertools::Itertools;

use crate::{error::Result, Solution};

pub fn run(input: &str) -> Result<Solution> {
    let calorie_groups = parse(input)?;

    let sums: Vec<u32> = calorie_groups
        .iter()
        .map(|group| group.iter().sum::<u32>())
        .collect();

    let largest_sum = sums.iter().max().unwrap();

    let sum_of_top_three: u32 = sums.iter().sorted().rev().take(3).sum();

    Ok(Solution {
        part_one: largest_sum.to_string(),
        part_two: sum_of_top_three.to_string(),
    })
}

fn parse(input: &str) -> Result<Vec<Vec<u32>>> {
    Ok(input
        .trim()
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.trim())
                .map(|line| line.parse::<u32>().map_err(|err| err.into()))
                .collect()
        })
        .collect::<Result<_>>()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_examples() {
        let example = r"
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
        ";

        let solution = run(example).unwrap();

        assert_eq!(solution.part_one, "24000");
        assert_eq!(solution.part_two, "45000");
    }

    #[test]
    fn matches_offical_result() {
        let input = include_str!("./input/day01");

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "75622");
        assert_eq!(solution.part_two, "213159");
    }
}
