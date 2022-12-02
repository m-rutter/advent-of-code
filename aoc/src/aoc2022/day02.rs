use std::str::FromStr;

use crate::{error::AoCError, error::Result, Solution};

pub fn run(input: &str) -> Result<Solution> {
    let rounds: Vec<Round> = input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| line.parse())
        .collect::<Result<_>>()?;

    let stratagems: Vec<Stratagem> = rounds.iter().map(|line| line.into()).collect();

    let total_score = rounds.iter().map(|round| round.score()).sum::<u32>();
    let total_score_by_stratagems = stratagems
        .iter()
        .map(|stratagem| stratagem.score())
        .sum::<u32>();

    Ok(Solution {
        part_one: total_score.to_string(),
        part_two: total_score_by_stratagems.to_string(),
    })
}

#[derive(PartialEq, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

struct Round {
    opener: Move,
    response: Move,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

struct Stratagem {
    opener: Move,
    outcome: Outcome,
}

impl Round {
    fn score(&self) -> u32 {
        let base = self.response.score();

        let outcome = match (&self.opener, &self.response) {
            (opener, response) if opener == response => 3,
            (Move::Rock, Move::Paper) => 6,
            (Move::Paper, Move::Scissors) => 6,
            (Move::Scissors, Move::Rock) => 6,
            _ => 0,
        };

        return base + outcome;
    }
}

impl Stratagem {
    fn score(&self) -> u32 {
        let base = match (&self.opener, &self.outcome) {
            (Move::Rock, Outcome::Win)
            | (Move::Paper, Outcome::Draw)
            | (Move::Scissors, Outcome::Lose) => Move::Paper.score(),

            (Move::Rock, Outcome::Lose)
            | (Move::Paper, Outcome::Win)
            | (Move::Scissors, Outcome::Draw) => Move::Scissors.score(),

            (Move::Rock, Outcome::Draw)
            | (Move::Paper, Outcome::Lose)
            | (Move::Scissors, Outcome::Win) => Move::Rock.score(),
        };

        let outcome = match self.outcome {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        };

        base + outcome
    }
}

impl From<&Round> for Stratagem {
    fn from(round: &Round) -> Self {
        let outcome = match round.response {
            Move::Rock => Outcome::Lose,
            Move::Paper => Outcome::Draw,
            Move::Scissors => Outcome::Win,
        };

        Self {
            opener: round.opener.clone(),
            outcome: outcome,
        }
    }
}

impl FromStr for Round {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let pair: Vec<&str> = s.split_whitespace().collect();
        if pair.len() != 2 {
            return Err(crate::error::ParsingError::ParseError.into());
        }

        let opener = match pair[0] {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => return Err(crate::error::ParsingError::ParseError.into()),
        };

        let response = match pair[1] {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => return Err(crate::error::ParsingError::ParseError.into()),
        };

        Ok(Round {
            opener: opener,
            response: response,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn matches_examples() {
        let example = r"
            A Y
            B X
            C Z
        ";

        let solution = run(example).unwrap();

        assert_eq!(solution.part_one, "15");
        assert_eq!(solution.part_two, "12");
    }

    #[test]
    fn matches_offical_results() {
        let input = include_str!("./input/day02");

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "9177");
        assert_eq!(solution.part_two, "12111");
    }
}
