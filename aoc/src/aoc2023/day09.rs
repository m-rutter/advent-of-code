use itertools::Itertools;

use crate::{error::Result, Solution};

pub fn run(input: &str) -> Result<Solution> {
    let readings: Vec<Vec<i64>> = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|s| s.parse::<i64>().map_err(|err| err.into()))
                .collect::<Result<_>>()
        })
        .collect::<Result<_>>()?;

    let part_one: i64 = readings.iter().map(|reading| extrapolate(reading)).sum();

    let part_two: i64 = readings
        .into_iter()
        .map(|reading| {
            let reversed_reading = reading.into_iter().rev().collect_vec();

            extrapolate(&reversed_reading)
        })
        .sum();

    Ok(Solution {
        part_one: part_one.to_string(),
        part_two: part_two.to_string(),
    })
}

fn extrapolate(readings: &[i64]) -> i64 {
    let diffs = compute_diffs(readings);
    let last = *readings.last().expect("slice not to be empty");

    let is_all_zero = diffs.iter().all(|value| *value == 0);

    if is_all_zero {
        last
    } else {
        last + extrapolate(&diffs)
    }
}

fn compute_diffs(readings: &[i64]) -> Vec<i64> {
    readings
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offical_example_1() {
        let input = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let solution = run(&input).unwrap();

        assert_eq!(solution.part_one, "114");
        assert_eq!(solution.part_two, "2");
    }

    #[test]
    fn test_offical_input() {
        let input = include_str!("./input/day09");

        let solution = run(&input).unwrap();

        assert_eq!(solution.part_one, "1684566095");
        assert_eq!(solution.part_two, "1136");
    }
}
