use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{Solution, error::Result};

type Board = Vec<Vec<char>>;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"XMAS|SAMX").expect("Error compiling regex"));

pub fn run(input: &str) -> Result<Solution> {
    let board: Board = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect_vec())
        .collect();

    let rows: usize = board
        .iter()
        .map(|row| {
            let row: String = row.iter().collect();
            RE.captures_iter(&row).count()
        })
        .sum();

    let cols: usize = (0..board[0].len())
        .map(|col| {
            let column: String = board.iter().map(|row| row[col]).collect();
            RE.captures_iter(&column).count()
        })
        .sum();

    Ok(Solution {
        part_one: (rows + cols).to_string(),
        part_two: "".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn offical_example() {
        let input = "MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX";

        let result = run(input).unwrap();

        assert_eq!(result.part_one, "18");
        assert_eq!(result.part_two, "");
    }

    #[test]
    #[ignore]
    fn offical_input() {
        let input = include_str!("./input/day04");

        let result = run(input).unwrap();

        assert_eq!(result.part_one, "");
        assert_eq!(result.part_two, "");
    }
}
