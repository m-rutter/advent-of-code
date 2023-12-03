use std::collections::{HashMap, HashSet};

use crate::{error::Result, Solution};

pub fn run(input: &str) -> Result<Solution> {
    let grid: Grid = input
        .trim()
        .lines()
        .map(|line| line.trim().chars())
        .map(|chars| chars.map(|c| SchematicItem::from(c)))
        .map(|row| row.collect())
        .collect();

    let part_one = solve_part_one(&grid)?;

    Ok(Solution {
        part_one: part_one.to_string(),
        part_two: "".to_string(),
    })
}

fn solve_part_one(grid: &Grid) -> Result<usize> {
    let mut symbol_positions: HashSet<(isize, isize)> = HashSet::new();
    let mut number_spans: HashMap<(isize, (isize, isize)), usize> = HashMap::new();

    for (row_num, row) in grid.iter().enumerate() {
        let mut num_chars = vec![];
        let mut start_col: Option<isize> = None;

        for (col_num, col) in row.iter().enumerate() {
            if let SchematicItem::Symbol(_) = col {
                symbol_positions.insert((row_num as isize, col_num as isize));
            }

            if let SchematicItem::Num(c) = col {
                if start_col.is_none() {
                    start_col = Some(col_num as isize);
                }

                num_chars.push(c);
            } else {
                if let Some(start_col) = start_col {
                    let s: String = num_chars.clone().into_iter().collect();

                    let n = s.parse()?;

                    number_spans.insert((row_num as isize, (start_col, col_num as isize)), n);
                }

                start_col = None;
                num_chars = vec![];
            }
        }

        if let Some(col) = start_col {
            let s: String = num_chars.clone().into_iter().collect();

            let n = s.parse()?;

            number_spans.insert((row_num as isize, (col, row.len() as isize - 1)), n);
        }
    }

    let parts = number_spans.iter().filter(|span| {
        let (row, (start, end)) = span.0;

        for col in *start..*end {
            let row = *row;
            let adjacent_cells = [
                (row - 1, col),
                (row - 1, col + 1),
                (row - 1, col - 1),
                (row + 1, col),
                (row + 1, col + 1),
                (row + 1, col - 1),
                (row, col + 1),
                (row, col - 1),
            ];

            for (col, row) in adjacent_cells {
                if symbol_positions.contains(&(col, row)) {
                    return true;
                }
            }
        }

        false
    });

    Ok(parts.map(|part| part.1).sum())
}

type Grid = Vec<Vec<SchematicItem>>;

enum SchematicItem {
    FullStop,
    Num(char),
    Symbol(char),
}

impl From<char> for SchematicItem {
    fn from(value: char) -> Self {
        match value {
            '.' => SchematicItem::FullStop,
            c if c.is_digit(10) => SchematicItem::Num(c),
            _ => SchematicItem::Symbol(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_examples() {
        let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "4361");
    }

    #[test]
    fn match_offical_input() {
        let input = include_str!("./input/day03");

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "550934");
    }
}
