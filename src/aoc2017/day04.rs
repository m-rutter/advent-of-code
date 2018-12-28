use std::collections::HashSet;

use crate::{error, Solution};

/// Compute the solution to day 4 of AoC 2017
pub fn run(input: &str) -> error::AoCResult<Solution> {
    let input = parser(&input);

    let valid_passprase_count = valid_passprase_count(&input);
    let valid_passprase_count_no_anagrams = valid_passprase_anagrams(&input);

    Ok(Solution {
        part_one: valid_passprase_count.to_string(),
        part_two: valid_passprase_count_no_anagrams.to_string(),
    })
}

fn valid_passprase_anagrams(v: &[Vec<String>]) -> u32 {
    v.iter()
        .map(|passprase| {
            let pass_len = passprase.len();

            let set_len = passprase
                .iter()
                .map(|word| {
                    let mut chars = word.chars().collect::<Vec<char>>();
                    chars.sort();

                    chars
                })
                .collect::<HashSet<Vec<char>>>()
                .len();

            if set_len == pass_len {
                1
            } else {
                0
            }
        })
        .sum()
}

fn valid_passprase_count(v: &[Vec<String>]) -> u32 {
    v.iter()
        .map(|passprase| {
            let mut set = HashSet::new();
            let mut pass = 1;

            for word in passprase.iter() {
                if set.contains(word) {
                    pass = 0;
                    break;
                } else {
                    set.insert(word);
                }
            }

            pass
        })
        .sum()
}

fn parser(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.to_string())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_offical_result() {
        let input = include_str!("./input/day04");

        let result = run(&input).unwrap();

        assert_eq!(result.part_one, "325");
        assert_eq!(result.part_two, "119");
    }

}
