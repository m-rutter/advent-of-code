use failure::Error;
use std::collections::{HashMap, HashSet};
use std::ops::Add;
use std::process;

use super::{AoCError, AoCSolution};

/// Compute the solution to day 3 of AoC 2017
pub fn run(input: &str) -> Result<AoCSolution, AoCError> {
    let input = parser(&input).unwrap_or_else(|err| {
        eprintln!("Problem parsing input: {}", err);
        process::exit(1);
    });

    let part_one = distance(input);
    let part_two = memory_walk(input);

    Ok(AoCSolution {
        part_one: part_one.to_string(),
        part_two: part_two.to_string(),
    })
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Cell {
    x: i64,
    y: i64,
}

impl Cell {
    fn new(x: i64, y: i64) -> Cell {
        Cell { x, y }
    }

    fn get_adjacent(c: Cell) -> Vec<Cell> {
        vec![
            Cell::new(1, 0),
            Cell::new(1, 1),
            Cell::new(0, 1),
            Cell::new(-1, 1),
            Cell::new(-1, 0),
            Cell::new(-1, -1),
            Cell::new(0, -1),
            Cell::new(1, -1),
        ]
        .iter()
        .map(|&a| a + c)
        .collect()
    }
}

impl Add for Cell {
    type Output = Cell;

    fn add(self, other: Self) -> Cell {
        Cell {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

enum Orientation {
    East,
    North,
    West,
    South,
}

impl Orientation {
    fn look_left(&self, position: Cell) -> Cell {
        match self {
            Orientation::East => Cell::new(0, 1) + position,
            Orientation::North => Cell::new(-1, 0) + position,
            Orientation::West => Cell::new(0, -1) + position,
            Orientation::South => Cell::new(1, 0) + position,
        }
    }

    fn move_to(&self, position: Cell) -> Cell {
        match self {
            Orientation::East => Cell::new(1, 0) + position,
            Orientation::North => Cell::new(0, 1) + position,
            Orientation::West => Cell::new(-1, 0) + position,
            Orientation::South => Cell::new(0, -1) + position,
        }
    }

    fn orient_left(&self) -> Orientation {
        match self {
            Orientation::East => Orientation::North,
            Orientation::North => Orientation::West,
            Orientation::West => Orientation::South,
            Orientation::South => Orientation::East,
        }
    }
}

fn distance(limit: u64) -> u64 {
    let mut direction = Orientation::East;
    let mut matrix = HashSet::new();

    matrix.insert(Cell::new(0, 0));

    let mut previous = Cell::new(1, 0);
    matrix.insert(previous);

    let range = 2..limit;

    for _ in range {
        let look_at = direction.look_left(previous);

        if matrix.contains(&look_at) {
            let next_step = direction.move_to(previous);

            matrix.insert(next_step);

            previous = next_step;
        } else {
            matrix.insert(look_at);

            previous = look_at;
            direction = direction.orient_left();
        };
    }

    (previous.x.abs() + previous.y.abs()) as u64
}

#[allow(clippy::map_entry)]
fn memory_walk(limit: u64) -> u64 {
    let mut direction = Orientation::East;
    let mut matrix: HashMap<Cell, u64> = HashMap::new();

    matrix.insert(Cell::new(0, 0), 1);

    let mut previous = Cell::new(1, 0);
    matrix.insert(previous, 1);

    let range = 2..limit;

    for _ in range {
        let look_at = direction.look_left(previous);

        if matrix.contains_key(&look_at) {
            let next_step = direction.move_to(previous);
            let value = Cell::get_adjacent(next_step)
                .iter()
                .filter_map(|c| matrix.get(c))
                .sum();

            matrix.insert(next_step, value);
            previous = next_step;

            if value > limit {
                break;
            }
        } else {
            let value = Cell::get_adjacent(look_at)
                .iter()
                .filter_map(|c| matrix.get(c))
                .sum();

            matrix.insert(look_at, value);

            previous = look_at;
            direction = direction.orient_left();

            if value > limit {
                break;
            }
        };
    }

    matrix[&previous]
}

fn parser(input: &str) -> Result<u64, Error> {
    let num = input.trim().parse()?;

    Ok(num)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_offical_result() {
        let input = include_str!("./input");

        let result = run(&input).unwrap();

        assert_eq!(result.part_one, "438");
        assert_eq!(result.part_two, "266330");
    }
}
