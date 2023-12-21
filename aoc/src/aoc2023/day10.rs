use std::str::FromStr;

use anyhow::anyhow;
use strum::IntoEnumIterator;

use crate::{
    error::{AoCError, Result},
    Solution,
};

pub fn run(input: &str) -> Result<Solution> {
    let grid = PipeGrid::from_str(input)?;

    let len = grid.pipe_len();

    Ok(Solution {
        part_one: (len / 2).to_string(),
        part_two: "".to_string(),
    })
}

#[derive(Debug)]
struct PipeGrid {
    grid: Vec<Vec<Pipe>>,
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    y: usize,
    x: usize,
}

impl Pos {
    fn next_coodindate(&self, direction: &Direction) -> Option<Pos> {
        let pos = self;
        Some(match direction {
            Direction::South => Pos {
                y: pos.y + 1,
                x: pos.x,
            },
            Direction::East => Pos {
                y: pos.y,
                x: pos.x + 1,
            },
            Direction::North => {
                if pos.y < 1 {
                    return None;
                } else {
                    Pos {
                        y: pos.y - 1,
                        x: pos.x,
                    }
                }
            }
            Direction::West => {
                if pos.x < 1 {
                    return None;
                } else {
                    Pos {
                        y: pos.y,
                        x: pos.x - 1,
                    }
                }
            }
        })
    }
}

impl PipeGrid {
    fn pipe_len(&self) -> u64 {
        let (_, start_pos) = self.find_start().unwrap();

        let (direction, pipe, pos) = self.find_connecting_pipes(start_pos)[0];

        let mut direction = direction;
        let mut pipe = pipe;
        let mut pos = pos;

        let mut count = 1;

        loop {
            let next_pipe_direction = pipe.next_pipe_direction(direction).unwrap();
            let next_pipe_pos = pos.next_coodindate(&next_pipe_direction).unwrap();
            let next_pipe = self.get_pipe(next_pipe_pos);

            count += 1;
            if next_pipe == Pipe::Start {
                break count;
            }

            direction = next_pipe_direction;
            pipe = next_pipe;
            pos = next_pipe_pos;
        }
    }

    fn get_pipe(&self, pos: Pos) -> Pipe {
        self.grid[pos.y][pos.x]
    }

    fn find_start(&self) -> Option<(Pipe, Pos)> {
        self.grid.iter().enumerate().find_map(|(y, row)| {
            let pipe = row
                .iter()
                .enumerate()
                .find(|(_, pipe)| **pipe == Pipe::Start);

            if let Some((x, pipe)) = pipe {
                Some((*pipe, Pos { x, y }))
            } else {
                None
            }
        })
    }

    fn find_connecting_pipes(&self, pos: Pos) -> Vec<(Direction, Pipe, Pos)> {
        let mut connecting_pos = vec![];

        for direction in Direction::iter() {
            if let Some(next_coodindate) = pos.next_coodindate(&direction) {
                let pipe = self
                    .grid
                    .get(next_coodindate.y)
                    .and_then(|row| row.get(next_coodindate.x));

                if let Some(pipe) = pipe {
                    let is_valid = pipe.is_valid_connecting_pipe(direction);

                    if is_valid {
                        connecting_pos.push((direction, *pipe, next_coodindate));
                    }
                }
            }
        }

        connecting_pos
    }
}

impl FromStr for PipeGrid {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let grid: Vec<Vec<Pipe>> = s
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| Pipe::try_from(c))
                    .collect::<Result<_>>()
            })
            .collect::<Result<_>>()?;

        let grid_size = grid.len();

        if grid_size < 2 {
            return Err(anyhow!("expected square grid of at least size 2x2").into());
        }

        if grid.iter().all(|row| row.len() != grid_size) {
            return Err(
                anyhow!("expected square grid, not all rows match col of {grid_size}").into(),
            );
        }

        Ok(Self { grid })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

#[derive(Debug, PartialEq, strum::EnumIter, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Pipe {
    fn is_valid_connecting_pipe(&self, origin: Direction) -> bool {
        let pipe = self;

        match origin {
            Direction::North => match pipe {
                Pipe::Vertical | Pipe::SouthWest | Pipe::SouthEast | Pipe::Start => true,
                _ => false,
            },
            Direction::South => match pipe {
                Pipe::Vertical | Pipe::NorthWest | Pipe::NorthEast | Pipe::Start => true,
                _ => false,
            },
            Direction::East => match pipe {
                Pipe::Horizontal | Pipe::SouthWest | Pipe::NorthWest | Pipe::Start => true,
                _ => false,
            },
            Direction::West => match pipe {
                Pipe::Horizontal | Pipe::SouthEast | Pipe::NorthEast | Pipe::Start => true,
                _ => false,
            },
        }
    }

    fn next_pipe_direction(&self, origin: Direction) -> Option<Direction> {
        use Direction::*;

        Some(match self {
            Pipe::Vertical => match origin {
                North => North,
                South => South,
                _ => return None,
            },
            Pipe::Horizontal => match origin {
                East => East,
                West => West,
                _ => return None,
            },
            Pipe::NorthEast => match origin {
                South => East,
                West => North,
                _ => return None,
            },
            Pipe::NorthWest => match origin {
                South => West,
                East => North,
                _ => return None,
            },
            Pipe::SouthWest => match origin {
                North => West,
                East => South,
                _ => return None,
            },
            Pipe::SouthEast => match origin {
                North => East,
                West => South,
                _ => return None,
            },
            // start and ground don't really fit here
            _ => return None,
        })
    }
}

impl TryFrom<char> for Pipe {
    type Error = AoCError;

    fn try_from(value: char) -> Result<Self> {
        use Pipe::*;

        Ok(match value {
            '|' => Vertical,
            '-' => Horizontal,
            'L' => NorthEast,
            'J' => NorthWest,
            '7' => SouthWest,
            'F' => SouthEast,
            '.' => Ground,
            'S' => Start,
            _ => return Err(anyhow!("not a recongised pipe segment, {value}").into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offical_example_1() {
        let input = r"-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

        let solution = run(&input).unwrap();

        assert_eq!(solution.part_one, "4");
        assert_eq!(solution.part_two, "");
    }

    #[test]
    fn test_offical_input() {
        let input = include_str!("./input/day10");

        let solution = run(&input).unwrap();

        assert_eq!(solution.part_one, "7030");
        assert_eq!(solution.part_two, "");
    }
}
