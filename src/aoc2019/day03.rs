use std::{collections::HashMap, convert::TryFrom};

use once_cell::sync::Lazy;
use regex::Regex;

use crate::error;
use crate::Solution;

pub fn run(input: &str) -> error::AoCResult<Solution> {
    let movements = parse(input);

    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();

    for wire in movements.iter() {
        let mut current_pos = (0, 0);
        for movement in wire.iter() {
            let next_pos: (i32, i32);

            match movement {
                Movement::Left(distance) => {
                    next_pos = (current_pos.0 - distance, current_pos.1);

                    for x in next_pos.0..current_pos.0 {
                        *grid.entry((x, current_pos.1)).or_insert(0) += 1;
                    }
                }
                Movement::Up(distance) => {
                    next_pos = (current_pos.0, current_pos.1 + distance);
                    for y in next_pos.1..current_pos.1 {
                        *grid.entry((y, current_pos.1)).or_insert(0) += 1;
                    }
                }
                Movement::Right(distance) => {
                    next_pos = (current_pos.0 + distance, current_pos.1);
                    for x in next_pos.0..current_pos.0 {
                        *grid.entry((x, current_pos.1)).or_insert(0) += 1;
                    }
                }
                Movement::Down(distance) => {
                    next_pos = (current_pos.0, current_pos.1 - distance);
                    for y in next_pos.1..current_pos.1 {
                        *grid.entry((y, current_pos.1)).or_insert(0) += 1;
                    }
                }
            }

            current_pos = next_pos;
        }
    }

    println!("{:?}", grid);

    unimplemented!()
}

fn parse(input: &str) -> Vec<Vec<Movement>> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|s| Movement::try_from(s).ok())
                .collect::<Vec<Movement>>()
        })
        .collect()
}

#[derive(Debug)]
enum Movement {
    Left(i32),
    Up(i32),
    Right(i32),
    Down(i32),
}

static MOVEMENT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<direction>[L,U,R,D])(?P<distance>[\d]*)").expect("Error compiling regex")
});

impl TryFrom<&str> for Movement {
    type Error = error::Error;
    fn try_from(s: &str) -> Result<Movement, Self::Error> {
        let captures = match MOVEMENT_RE.captures(s) {
            Some(captures) => captures,
            None => Err(error::Error::msg(&"regex error"))?,
        };

        let direction = &captures["direction"];
        let distance = *&captures["distance"]
            .parse::<i32>()
            .expect("Unable to parse number from regex number capture");

        Ok(match direction {
            "L" => Movement::Left(distance),
            "U" => Movement::Up(distance),
            "R" => Movement::Right(distance),
            "D" => Movement::Down(distance),
            _ => unreachable!(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn original_examples() {
        let input = "R8,U5,L5,D3\nU7,R6,D4,L4";

        run(input);
    }

    #[test]
    fn test_regex() {
        let captures1 = MOVEMENT_RE.captures("U123").unwrap();
        let captures2 = MOVEMENT_RE.captures("R42342342").unwrap();
        let captures3 = MOVEMENT_RE.captures("D43242342").unwrap();
        let captures4 = MOVEMENT_RE.captures("L426621321").unwrap();

        assert_eq!("U", &captures1["direction"]);
        assert_eq!("123", &captures1["distance"]);

        assert_eq!("R", &captures2["direction"]);
        assert_eq!("42342342", &captures2["distance"]);

        assert_eq!("D", &captures3["direction"]);
        assert_eq!("43242342", &captures3["distance"]);

        assert_eq!("L", &captures4["direction"]);
        assert_eq!("426621321", &captures4["distance"]);
    }
}
