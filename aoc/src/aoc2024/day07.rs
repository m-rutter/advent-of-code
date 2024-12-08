use std::str::FromStr;

use crate::{
    error::{AoCError, Result},
    Solution,
};

pub fn run(input: &str) -> Result<Solution> {
    let calibrations: Vec<Calibration> = input
        .trim()
        .lines()
        .map(|line| line.trim().parse())
        .collect::<Result<_>>()?;

    calibrations[1].is_solvable(false);

    let part_one: u64 = calibrations
        .iter()
        .filter(|cal| cal.is_solvable(false))
        .map(|cal| cal.target)
        .sum();

    let part_two: u64 = calibrations
        .iter()
        .filter(|cal| cal.is_solvable(true))
        .map(|cal| cal.target)
        .sum();

    Ok(Solution {
        part_one: part_one.to_string(),
        part_two: part_two.to_string(),
    })
}

#[derive(Debug)]
struct Calibration {
    target: u64,
    operands: Vec<u64>,
}

impl Calibration {
    fn is_solvable(&self, with_concat: bool) -> bool {
        let initial = self.operands[0];

        return Calibration::_is_solvable(initial, self.target, &self.operands[1..], with_concat);
    }

    fn _is_solvable(acc: u64, target: u64, operands: &[u64], with_concat: bool) -> bool {
        if operands.is_empty() && acc != target {
            return false;
        } else if operands.is_empty() && acc == target {
            return true;
        }

        let mut add_acc = acc;
        add_acc += operands[0];

        if Calibration::_is_solvable(add_acc, target, &operands[1..], with_concat) {
            return true;
        }

        let mut mul_acc = acc;
        mul_acc *= operands[0];

        if Calibration::_is_solvable(mul_acc, target, &operands[1..], with_concat) {
            return true;
        }

        if with_concat {
            let next = operands[0];

            let concat_acc = format!("{acc}{next}").parse::<u64>().unwrap();

            return Calibration::_is_solvable(concat_acc, target, &operands[1..], with_concat);
        } else {
            return false;
        }
    }
}

impl FromStr for Calibration {
    type Err = AoCError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = s.split(":");

        let target = parts.next().unwrap().parse::<u64>()?;

        let operands: Vec<u64> = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u64>().map_err(|err| err.into()))
            .collect::<Result<_>>()?;

        Ok(Calibration { target, operands })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offical_example() {
        let input = r"190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20";

        let result = run(input).unwrap();

        assert_eq!(result.part_one, "3749");
        assert_eq!(result.part_two, "11387");
    }

    #[test]
    fn offical_input() {
        let input = include_str!("./input/day07");

        let result = run(input).unwrap();

        assert_eq!(result.part_one, "303766880536");
        assert_eq!(result.part_two, "337041851384440");
    }
}
