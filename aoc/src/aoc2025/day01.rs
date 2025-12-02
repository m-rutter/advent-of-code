use std::str::FromStr;

use anyhow::anyhow;

use crate::{
    Solution,
    error::{AoCError, Result},
};

pub fn run(input: &str) -> Result<Solution> {
    let instructions: Vec<Instruction> = input
        .trim()
        .lines()
        .map(|line| line.trim().parse())
        .collect::<Result<_>>()?;

    let mut pos = 50;
    let mut zero_pos_count = 0;
    for Instruction { direction, turn } in instructions.iter() {
        match direction {
            Direction::R => {
                pos = (pos + turn) % 100;
            }
            Direction::L => {
                pos = (100 + pos - turn % 100) % 100;
            }
        }

        if pos == 0 {
            zero_pos_count += 1;
        }
    }

    // I'm sure there must be a smarter way of doing this, but this works
    let mut zero_click_count = 0;
    let mut dial = 50;
    for Instruction { direction, turn } in instructions.iter() {
        for _ in 0..*turn {
            if let Direction::R = direction {
                dial += 1;
            } else {
                dial -= 1;
            }

            if dial > 99 {
                dial = 0
            }

            if dial < 0 {
                dial = 99
            }

            if dial == 0 {
                zero_click_count += 1;
            }
        }
    }

    Ok(Solution {
        part_one: zero_pos_count.to_string(),
        part_two: zero_click_count.to_string(),
    })
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    turn: i32,
}

#[derive(Debug)]
enum Direction {
    L,
    R,
}

impl FromStr for Instruction {
    type Err = AoCError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut chars = s.chars();
        let direction = chars.next();
        let movement: i32 = String::from_iter(chars).parse()?;

        Ok(match direction {
            Some('L') => Self {
                direction: Direction::L,
                turn: movement,
            },
            Some('R') => Self {
                direction: Direction::R,
                turn: movement,
            },
            _ => return Err(anyhow!("expected 'L' or 'R' char at start of each line").into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offical_example_1() {
        let input = r"L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82";

        let result = run(input).unwrap();

        assert_eq!(result.part_one, "3");
        assert_eq!(result.part_two, "6");
    }

    #[test]
    fn test_zoey_example() {
        let input = r"L50
        R200
        L50
        R100
        L175";

        let result = run(input).unwrap();

        assert_eq!(result.part_one, "2");
        assert_eq!(result.part_two, "6");
    }

    #[test]
    fn test_offical_input() {
        let input = include_str!("./input/day01");

        let solution = run(&input).unwrap();

        assert_eq!(solution.part_one, "989");
        assert_eq!(solution.part_two, "5941");
    }
}
