use anyhow::anyhow;
use itertools::Itertools;

use crate::{error::Result, Solution};

pub fn run(input: &str) -> Result<Solution> {
    let mut list1 = vec![];
    let mut list2 = vec![];

    for (line_idx, line) in input.trim().lines().enumerate() {
        for (col_idx, location) in line.trim().split_whitespace().enumerate() {
            let location = location.parse::<usize>()?;

            if col_idx == 0 {
                list1.push(location);
                continue;
            } else if col_idx == 1 {
                list2.push(location);
                continue;
            }

            return Err(anyhow!(
                "Unexcepted extra input on line {line_idx} col {col_idx}"
            ))?;
        }
    }

    let part_one: usize = list1
        .iter()
        .sorted()
        .zip(list2.iter().sorted())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();

    let part_two: usize = list1
        .iter()
        .map(|a| a * list2.iter().filter(|b| *a == **b).count())
        .sum();

    Ok(Solution {
        part_one: part_one.to_string(),
        part_two: part_two.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offical_example_1() {
        let input = r"3   4
        4   3
        2   5
        1   3
        3   9
        3   3";

        let result = run(input).unwrap();

        assert_eq!(result.part_one, "11");
    }

    #[test]
    fn test_offical_input() {
        let input = include_str!("./input/day01");

        let solution = run(&input).unwrap();

        assert_eq!(solution.part_one, "1722302");
        assert_eq!(solution.part_two, "20373490");
    }
}
