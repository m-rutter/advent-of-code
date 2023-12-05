use std::{collections::HashSet, usize};

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
    let part_two = solve_part_two(&grid)?;

    Ok(Solution {
        part_one: part_one.to_string(),
        part_two: part_two.to_string(),
    })
}

#[derive(Debug)]
struct Part {
    number: usize,
    positions: HashSet<(usize, usize)>,
}

fn solve_part_one(grid: &Grid) -> Result<usize> {
    let parts = get_parts(grid)?;

    Ok(parts.iter().map(|part| part.number).sum())
}

fn solve_part_two(grid: &Grid) -> Result<usize> {
    let parts = get_parts(grid)?;
    let mut gear_raitos = vec![];

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if let SchematicItem::Symbol('*') = col {
                let adjacent_cells = (
                    row_index.saturating_sub(1)..row_index + 2,
                    col_index.saturating_sub(1)..col_index + 2,
                );

                let parts: Vec<_> = parts
                    .iter()
                    .filter(|part| {
                        for ri in adjacent_cells.0.clone() {
                            for ci in adjacent_cells.1.clone() {
                                if part.positions.contains(&(ri, ci)) {
                                    return true;
                                }
                            }
                        }

                        false
                    })
                    .collect();

                if parts.len() == 2 {
                    gear_raitos.push(parts[0].number * parts[1].number);
                }
            }
        }
    }

    Ok(gear_raitos.iter().sum())
}

fn get_parts(grid: &Vec<Vec<SchematicItem>>) -> Result<Vec<Part>> {
    let mut parts = vec![];
    for (row_index, row) in grid.iter().enumerate() {
        let mut num_chars = vec![];
        let mut start: Option<usize> = None;

        for (col_index, col) in row.iter().enumerate() {
            if let SchematicItem::Num(c) = col {
                if start.is_none() {
                    start = Some(col_index);
                }
                num_chars.push(c);
            } else if let Some(start_col) = start {
                let found = check_adjacent_cells(grid, row_index, start_col, col_index);

                if found {
                    let s: String = num_chars.clone().into_iter().collect();
                    let n: usize = s.parse()?;

                    let positions: HashSet<_> =
                        (start_col..col_index).map(|c| (row_index, c)).collect();

                    parts.push(Part {
                        number: n,
                        positions,
                    });
                }

                start = None;
                num_chars = vec![];
            }
        }

        if let Some(start_col) = start {
            let found = check_adjacent_cells(grid, row_index, start_col, row.len() - 1);

            if found {
                let s: String = num_chars.clone().into_iter().collect();
                let n: usize = s.parse()?;

                let positions: HashSet<_> =
                    (start_col..row.len() - 1).map(|c| (row_index, c)).collect();

                parts.push(Part {
                    number: n,
                    positions,
                });
            }
        }
    }
    Ok(parts)
}

fn check_adjacent_cells(grid: &Grid, row_index: usize, start_col: usize, end_col: usize) -> bool {
    let adjacent_cells = (
        row_index.saturating_sub(1)..row_index + 2,
        start_col.saturating_sub(1)..end_col + 1,
    );

    let mut found = false;
    for r in adjacent_cells.0 {
        for c in adjacent_cells.1.clone() {
            if let Some(SchematicItem::Symbol(_)) = grid.get(r).and_then(|row| row.get(c)) {
                found = true;
                break;
            }
        }
    }

    found
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
        assert_eq!(solution.part_two, "467835");
    }

    #[test]
    fn match_offical_input() {
        let input = include_str!("./input/day03");

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "550934");
        assert_eq!(solution.part_two, "81997870");
    }
}
