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
    let mut parts_numbers = vec![];

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
                let mut found = check_adjacent_cells(grid, row_index, start_col, col_index);

                if found {
                    let s: String = num_chars.clone().into_iter().collect();
                    let n: usize = s.parse()?;

                    parts_numbers.push(n);
                }

                start = None;
                num_chars = vec![];
            }
        }

        if let Some(start_col) = start {
            let mut found = check_adjacent_cells(grid, row_index, start_col, row.len() - 1);

            if found {
                let s: String = num_chars.clone().into_iter().collect();
                let n: usize = s.parse()?;
                parts_numbers.push(n);
            }
        }
    }

    Ok(parts_numbers.iter().sum())
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
    }

    #[test]
    fn match_offical_input() {
        let input = include_str!("./input/day03");

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "550934");
    }
}
