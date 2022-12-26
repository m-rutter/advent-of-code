use std::collections::HashSet;

use crate::{error::Result, Solution};

pub fn run(input: &str) -> Result<Solution> {
    let chars_with_index = input.chars().enumerate().collect::<Vec<(usize, char)>>();

    let part_one = chars_with_index
        .windows(4)
        .find_map(|slice| {
            let s = slice.iter().map(|(_, c)| *c).collect::<HashSet<_>>();

            if s.len() == 4 {
                return Some(slice.last().unwrap().0 + 1);
            }

            None
        })
        .ok_or_else(|| anyhow::anyhow!("unable to find marker character"))?;

    let part_two = chars_with_index
        .windows(14)
        .find_map(|slice| {
            let s = slice.iter().map(|(_, c)| *c).collect::<HashSet<_>>();

            if s.len() == 14 {
                return Some(slice.last().unwrap().0 + 1);
            }

            None
        })
        .ok_or_else(|| anyhow::anyhow!("unable to find marker character"))?;

    Ok(Solution {
        part_one: part_one.to_string(),
        part_two: part_two.to_string(),
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn matches_examples() {
        let examples = [
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];

        let solutions = examples.map(|s| run(s).unwrap());

        assert_eq!(
            solutions.clone().map(|s| s.part_one),
            ["7", "5", "6", "10", "11"]
        );

        assert_eq!(
            solutions.map(|s| s.part_two),
            ["19", "23", "23", "29", "26"]
        );
    }

    #[test]
    fn matches_offical_results() {
        let input = include_str!("./input/day06");

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "1953");
        assert_eq!(solution.part_two, "2301");
    }
}
