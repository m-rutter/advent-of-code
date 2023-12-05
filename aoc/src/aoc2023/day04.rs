use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    usize,
};

use anyhow::anyhow;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{
    error::{AoCError, Result},
    Solution,
};

pub fn run(input: &str) -> Result<Solution> {
    let cards: Vec<Scratchcard> = input
        .trim()
        .lines()
        .map(|line| line.trim().parse())
        .collect::<Result<_>>()?;

    let part_one: usize = cards.iter().map(|card| card.calculate_score()).sum();

    let mut pile_of_cards: HashMap<usize, usize> = HashMap::new();

    for card in cards {
        let count = pile_of_cards.entry(card.id).or_insert(1).clone();
        let wins = card.winning_count();

        for i in card.id + 1..=card.id + wins {
            let current_dupe_count = pile_of_cards.entry(i).or_insert(1);

            *current_dupe_count = *current_dupe_count + count;
        }
    }

    let part_two: usize = pile_of_cards.iter().map(|(_, v)| v).sum();

    Ok(Solution {
        part_one: part_one.to_string(),
        part_two: part_two.to_string(),
    })
}

#[derive(Debug, Clone)]
struct Scratchcard {
    id: usize,
    winning_numbers: HashSet<usize>,
    numbers: HashSet<usize>,
}

impl Scratchcard {
    fn winning_count(&self) -> usize {
        self.winning_numbers.intersection(&self.numbers).count()
    }

    fn calculate_score(&self) -> usize {
        let winning_number_count = self.winning_count();
        if winning_number_count == 0 {
            0
        } else {
            usize::pow(2, (winning_number_count - 1) as u32)
        }
    }
}

static CARD_NO_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"Card\s+(?P<id>\d+):").expect("unable to compile card no regex"));

static NUMBERS_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?P<num>\d+)").expect("unable to compile number regex"));

impl FromStr for Scratchcard {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let id: usize = CARD_NO_RE
            .captures(s)
            .and_then(|capture| capture.name("id"))
            .and_then(|name| Some(name.as_str()))
            .ok_or_else(|| anyhow!("Failed to parse game: {s}"))?
            .parse()?;

        let mut numbers = s
            .split(':')
            .last()
            .ok_or_else(|| anyhow!("failed to parse numbers: {s}"))?
            .split('|');

        let winning_numbers = numbers
            .next()
            .ok_or_else(|| anyhow!("failed to parse winning numbers: {s}"))?;

        let winning_numbers = NUMBERS_RE
            .captures_iter(winning_numbers)
            .map(|capture| capture.name("num").unwrap().as_str().parse().unwrap())
            .collect();

        let numbers = numbers
            .next()
            .ok_or_else(|| anyhow!("failed to parse numbers: {s}"))?;

        let numbers = NUMBERS_RE
            .captures_iter(numbers)
            .map(|capture| capture.name("num").unwrap().as_str().parse().unwrap())
            .collect();

        Ok(Scratchcard {
            id,
            numbers,
            winning_numbers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = r"Card   1: 66 90 67 76 55 13 91 31 95  4 | 82 98 69  8 15  2 32 24 99 56 46 65 60 72 58 68 54 22 26  5 74 25 84 73 61";

        let card: Scratchcard = input.parse().unwrap();

        assert_eq!(card.id, 1);
    }

    #[test]
    fn test_cal_score() {
        let inputs = [
            r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            r"Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            r"Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            r"Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            r"Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            r"Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        let cards: Vec<Scratchcard> = inputs.iter().map(|s| s.parse().unwrap()).collect();

        assert_eq!(cards[0].calculate_score(), 8);
        assert_eq!(cards[1].calculate_score(), 2);
        assert_eq!(cards[2].calculate_score(), 2);
        assert_eq!(cards[3].calculate_score(), 1);
        assert_eq!(cards[4].calculate_score(), 0);
        assert_eq!(cards[5].calculate_score(), 0);

        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let card: Scratchcard = input.parse().unwrap();

        assert_eq!(card.calculate_score(), 8);
    }

    #[test]
    fn match_examples() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "13");
        assert_eq!(solution.part_two, "30");
    }

    #[test]
    fn match_offical_input() {
        let input = include_str!("./input/day04");

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "27845");
        assert_eq!(solution.part_two, "9496801");
    }
}
