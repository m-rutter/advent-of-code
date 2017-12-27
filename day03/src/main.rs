extern crate failure;

use failure::Error;

use std::io::{self, Read};
use std::ops::Add;
use std::collections::HashSet;
use std::process;

fn main() {
    let input = parser().unwrap_or_else(|err| {
        eprintln!("Problem parsing input: {}", err);
        process::exit(1);
    });


    let part_one = distance(input);

    println!("Part one solution: {:?}", part_one);
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
}

impl Add for Cell {
    type Output = Cell;

    fn add(self, other: Cell) -> Cell {
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
            &Orientation::East => Cell::new(0, 1) + position,
            &Orientation::North => Cell::new(-1, 0) + position,
            &Orientation::West => Cell::new(0, -1) + position,
            &Orientation::South => Cell::new(1, 0) + position,
        }
    }

    fn move_to(&self, position: Cell) -> Cell {
        match self {
            &Orientation::East => Cell::new(1, 0) + position,
            &Orientation::North => Cell::new(0, 1) + position,
            &Orientation::West => Cell::new(-1, 0) + position,
            &Orientation::South => Cell::new(0, -1) + position,
        }
    }

    fn orient_left(&self) -> Orientation {
        match self {
            &Orientation::East => Orientation::North,
            &Orientation::North => Orientation::West,
            &Orientation::West => Orientation::South,
            &Orientation::South => Orientation::East,
        }
    }
}

fn distance(limit: u32) -> u32 {
    let mut direction = Orientation::East;
    let mut matrix = HashSet::new();
    let mut previous = Cell::new(1, 0);

    matrix.insert(Cell::new(0, 0));
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

    (previous.x + previous.y.abs()) as u32
}

fn parser() -> Result<u32, Error> {
    let mut stdin = io::stdin();

    let mut buff = String::new();

    stdin.read_to_string(&mut buff)?;

    let num = buff.parse()?;

    Ok(num)
}
