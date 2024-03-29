use std::str::FromStr;

use anyhow::anyhow;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{error, error::Result, Solution};

pub fn run(input: &str) -> Result<Solution> {
    let games: Vec<Game> = input
        .trim()
        .lines()
        .map(|line| line.trim().parse())
        .collect::<Result<_>>()?;

    let part_one: usize = games
        .iter()
        .filter(|game| {
            game.within_constraint(&Constraint {
                red: 12,
                green: 13,
                blue: 14,
            })
        })
        .map(|game| game.id)
        .sum();

    let part_two: usize = games
        .iter()
        .map(|game| game.smallest_constraint())
        .map(|constraint| constraint.red * constraint.green * constraint.blue)
        .sum();

    Ok(Solution {
        part_one: part_one.to_string(),
        part_two: part_two.to_string(),
    })
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug, strum::EnumString, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
enum ColorType {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Constraint {
    red: usize,
    green: usize,
    blue: usize,
}

impl Game {
    fn within_constraint(&self, constraint: &Constraint) -> bool {
        self.rounds.iter().all(|round| {
            round.red <= constraint.red
                && round.green <= constraint.green
                && round.blue <= constraint.blue
        })
    }

    fn smallest_constraint(&self) -> Constraint {
        let iter = self.rounds.iter();

        let red = iter.clone().map(|round| round.red).max().unwrap_or(0);
        let green = iter.clone().map(|round| round.green).max().unwrap_or(0);
        let blue = iter.clone().map(|round| round.blue).max().unwrap_or(0);

        Constraint { red, green, blue }
    }
}

static GAME_RE: Lazy<Regex> =
    Lazy::new(|| regex::Regex::new(r"Game (?P<id>\d+):").expect("Failed to compile game id regex"));

static ROUNDS_RE: Lazy<Regex> = Lazy::new(|| {
    regex::Regex::new(r"((?P<count>\d+) (?P<color>\w+),?\s?)")
        .expect("Failed to compile rounds regex")
});

impl FromStr for Game {
    type Err = error::AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let captures = GAME_RE
            .captures(s)
            .ok_or_else(|| anyhow!("Failed to parse game: {s}"))?;

        let id = captures.name("id").unwrap().as_str().parse()?;

        let mut rounds = vec![];

        for section in s.split(";") {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for capture in ROUNDS_RE.captures_iter(section) {
                let count = capture.name("count").unwrap().as_str().parse()?;

                let color = ColorType::from_str(capture.name("color").unwrap().as_str())
                    .map_err(|_| anyhow!("Failed to parse color for seciton: {section}"))?;

                match color {
                    ColorType::Red => {
                        red = count;
                    }
                    ColorType::Green => {
                        green = count;
                    }
                    ColorType::Blue => {
                        blue = count;
                    }
                }
            }

            rounds.push(Round { red, green, blue });
        }

        Ok(Game { id, rounds })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = r"Game 100: 14 green, 8 blue, 9 red; 5 blue, 4 green, 2 red; 4 red, 4 blue, 4 green; 1 blue, 3 green, 2 red; 10 red, 3 blue, 15 green; 2 red, 6 green, 3 blue";

        let game: Game = input.parse().unwrap();

        assert_eq!(game.id, 100);

        assert_eq!(game.rounds.len(), 6);

        assert_eq!(game.rounds[0].red, 9);
        assert_eq!(game.rounds[0].green, 14);
        assert_eq!(game.rounds[0].blue, 8);

        assert_eq!(game.rounds[1].red, 2);
        assert_eq!(game.rounds[1].green, 4);
        assert_eq!(game.rounds[1].blue, 5);
    }

    #[test]
    fn matches_example() {
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "8");
        assert_eq!(solution.part_two, "2286");
    }

    #[test]
    fn matches_offical_input() {
        let input = include_str!("./input/day02");

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "2256");
        assert_eq!(solution.part_two, "74229");
    }
}
