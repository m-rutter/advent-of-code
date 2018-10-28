use failure::Error;

use std::collections::HashSet;
use std::process;

use crate::{AoCError, AoCSolution};

/// Compute the solution to day 4 of AoC 2017
pub fn run(input: &str) -> Result<AoCSolution, AoCError> {
    let input = parser(&input).unwrap_or_else(|err| {
        eprintln!("Problem parsing input: {}", err);
        process::exit(1);
    });

    let valid_passprase_count = get_valid_passprase_count(&input);
    let valid_passprase_count_no_anagrams = get_valid_passprase_anagrams(&input);

    Ok(AoCSolution {
        part_one: valid_passprase_count.to_string(),
        part_two: valid_passprase_count_no_anagrams.to_string(),
    })
}

fn get_valid_passprase_anagrams(v: &[Vec<String>]) -> u32 {
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

fn get_valid_passprase_count(v: &[Vec<String>]) -> u32 {
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

fn parser(input: &str) -> Result<Vec<Vec<String>>, Error> {
    let passwords: Vec<Vec<String>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.to_string())
                .collect()
        })
        .collect();

    Ok(passwords)
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
