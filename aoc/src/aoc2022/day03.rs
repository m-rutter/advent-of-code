use itertools::Itertools;

use crate::{error::Result, Solution};
use std::collections::HashSet;

pub fn run(input: &str) -> Result<Solution> {
    let rucksacks = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>());

    let sum_of_common: u32 = rucksacks
        .clone()
        .map(|mut chars| {
            let right = chars.split_off(chars.len() / 2);
            let left = chars;

            char_score(find_common(&left, &right))
        })
        .sum();

    let sum_of_groups: u32 = rucksacks
        .batching(|it| {
            if let (Some(one), Some(two), Some(three)) = (it.next(), it.next(), it.next()) {
                let mut score = 0;
                for c in HashSet::<char>::from_iter(one.iter().cloned()) {
                    if two.contains(&c) && three.contains(&c) {
                        score = char_score(c);
                        break;
                    }
                }
                Some(score)
            } else {
                None
            }
        })
        .sum();

    Ok(Solution {
        part_one: sum_of_common.to_string(),
        part_two: sum_of_groups.to_string(),
    })
}

fn char_score(c: char) -> u32 {
    if c.is_uppercase() {
        (c as u8 - 38) as u32
    } else {
        (c as u8 - 96) as u32
    }
}

fn find_common(left: &[char], right: &[char]) -> char {
    let left_set: HashSet<&char> = HashSet::from_iter(left.iter());
    let right_set = HashSet::from_iter(right.iter());

    **left_set.intersection(&right_set).next().unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn matches_examples() {
        let example = r"
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        ";

        let solution = run(example).unwrap();

        assert_eq!(solution.part_one, "157");
        assert_eq!(solution.part_two, "70");
    }

    #[test]
    fn matches_offical_results() {
        let input = include_str!("./input/day03");

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "8493");
        assert_eq!(solution.part_two, "2552");
    }
}
