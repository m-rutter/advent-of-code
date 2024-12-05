use std::str::FromStr;

use itertools::Itertools;

use crate::{
    error::{AoCError, Result},
    Solution,
};

pub fn run(input: &str) -> Result<Solution> {
    let reports: Vec<Report> = input
        .trim()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| Report::from_str(line))
        .collect::<Result<_>>()?;

    let safe_reports_count = reports.iter().filter(|report| report.is_safe()).count();

    let damp_safe_reports_count = reports
        .iter()
        .filter(|report| report.is_damp_safe())
        .count();

    Ok(Solution {
        part_one: safe_reports_count.to_string(),
        part_two: damp_safe_reports_count.to_string(),
    })
}

#[derive(Debug)]
struct Report {
    levels: Vec<u32>,
}

impl Report {
    fn _is_safe(levels: &[u32]) -> bool {
        match levels.len() {
            1 => return true,
            2 => {
                if levels[0].abs_diff(levels[1]) > 3 {
                    return false;
                }
            }
            _ => {}
        };

        for (a, b, c) in levels.iter().tuple_windows() {
            let diff1 = (a).abs_diff(*b);
            let diff2 = b.abs_diff(*c);

            if diff1 > 3 || diff2 > 3 {
                return false;
            }

            if !(a > b && b > c) && !(a < b && b < c) {
                return false;
            }
        }

        true
    }

    fn is_safe(&self) -> bool {
        Report::_is_safe(&self.levels)
    }

    fn is_damp_safe(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        for to_skip in 0..self.levels.len() {
            let levels = self
                .levels
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != to_skip)
                .map(|(_, level)| *level)
                .collect_vec();

            if Report::_is_safe(&levels) {
                return true;
            }
        }

        false
    }
}

impl FromStr for Report {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let levels = s
            .trim()
            .split_whitespace()
            .map(|level| level.parse::<u32>().map_err(|err| err.into()))
            .collect::<Result<_>>()?;

        Ok(Report { levels })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offical_example() {
        let input = r"7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";

        let result = run(input).unwrap();

        assert_eq!(result.part_one, "2");
        assert_eq!(result.part_two, "4");
    }

    #[test]
    fn offical_input() {
        let input = include_str!("./input/day02");
        let result = run(input).unwrap();

        assert_eq!(result.part_one, "486");
        assert_eq!(result.part_two, "540");
    }
}
