use once_cell::sync::Lazy;
use regex::Regex;
use std::str::FromStr;

use crate::{
    error::{self, Result},
    Solution,
};

static MOVEMENT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<dir>forward|down|up)\s*(?P<dist>\d{1,})").expect("Error compiling regex")
});

#[derive(Debug, PartialEq, Eq)]
enum Movement {
    Up(u32),
    Down(u32),
    Forward(u32),
}

impl FromStr for Movement {
    type Err = error::AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let captures = MOVEMENT_RE
            .captures(s)
            .ok_or_else(|| error::ParsingError::ParseError)?;

        let dir = &captures["dir"];
        let dist: u32 = captures["dist"].parse()?;

        Ok(match dir {
            "forward" => Movement::Forward(dist),
            "up" => Movement::Up(dist),
            "down" => Movement::Down(dist),
            _ => Err(error::ParsingError::ParseError)?,
        })
    }
}

pub fn run(input: &str) -> Result<Solution> {
    let movements = parse(input)?;

    let (depth, distance) = movements
        .iter()
        .fold((0, 0), |(depth, distance), movement| match movement {
            Movement::Up(n) => (depth - n, distance),
            Movement::Down(n) => (depth + n, distance),
            Movement::Forward(n) => (depth, distance + n),
        });

    let part_one = (depth * distance).to_string();

    let (_, depth, distance) = movements.iter().fold(
        (0, 0, 0),
        |(aim, depth, distance), movement| match movement {
            Movement::Up(n) => (aim - n, depth, distance),
            Movement::Down(n) => (aim + n, depth, distance),
            Movement::Forward(n) => (aim, depth + (aim * n), distance + n),
        },
    );

    let part_two = (depth * distance).to_string();

    Ok(Solution {
        part_one: part_one,
        part_two: part_two,
    })
}

fn parse(input: &str) -> Result<Vec<Movement>> {
    input
        .trim()
        .lines()
        .map(|line| Movement::from_str(line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_test() {
        let input = r"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
        ";

        let movements = parse(input).unwrap();

        assert_eq!(
            movements,
            vec![
                Movement::Forward(5),
                Movement::Down(5),
                Movement::Forward(8),
                Movement::Up(3),
                Movement::Down(8),
                Movement::Forward(2)
            ]
        );
    }

    #[test]
    fn original_examples() {
        let input = r"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
        ";

        let result = run(&input).unwrap();

        assert_eq!(result.part_one, "150");
        assert_eq!(result.part_two, "900");
    }

    #[test]
    fn matches_offical_result() {
        let input = include_str!("./input/day02");

        let result = run(&input).unwrap();

        assert_eq!(result.part_one, "1762050");
        assert_eq!(result.part_two, "1855892637");
    }
}
