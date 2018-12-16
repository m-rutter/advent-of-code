use crate::{error, AoCSolution};
use failure::{format_err, Error};
use std::collections::HashMap;
use std::str::FromStr;

pub fn run(input: &str) -> error::AoCResult<AoCSolution> {
    let coordinates = parse(&input);

    unimplemented!()
}

fn parse(input: &str) -> Vec<Coordinates> {
    input
        .lines()
        .enumerate()
        .filter_map(|(id, line)| {
            let c = Coordinates::from_str(&line).ok();

            match c {
                Some(mut coordinates) => {
                    coordinates.id = Some(id as i32);
                    Some(coordinates)
                }
                None => None,
            }
        })
        .collect()
}

struct Grid<'a> {
    width: i32,
    height: i32,
    points: &'a [Coordinates],
    grid: HashMap<&'a Coordinates, ID>,
}

type ID = i32;

impl<'a> From<&'a [Coordinates]> for Grid<'a> {
    fn from(v: &'a [Coordinates]) -> Grid<'a> {
        let w = v.iter().map(|c| c.x).max_by(|a, b| a.cmp(&b)).unwrap_or(0);
        let h = v.iter().map(|c| c.y).max_by(|a, b| a.cmp(&b)).unwrap_or(0);

        let grid = HashMap::with_capacity((w * h) as usize);

        let x_range = 0..w;
        let y_range = 0..h;

        for x in x_range {
            for y in y_range.clone() {
                let c = Coordinates::new(x, y);
                let closest = c.find_closest(&v);
            }
        }

        Grid {
            width: w,
            height: h,
            points: v,
            grid: grid,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Coordinates {
    id: Option<i32>,
    x: i32,
    y: i32,
}

impl Coordinates {
    fn new(x: i32, y: i32) -> Self {
        Coordinates {
            x: x,
            y: y,
            id: None,
        }
    }

    fn distince_from(&self, other: &Coordinates) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn find_closest(&self, v: &[Coordinates]) -> Option<i32> {
        match v
            .into_iter()
            .map(|c| (self.distince_from(c), c))
            .min_by(|a, b| a.0.cmp(&b.0))
        {
            Some((_, c)) => c.id,
            None => None,
        }
    }
}

impl FromStr for Coordinates {
    type Err = Error;

    fn from_str(s: &str) -> Result<Coordinates, Error> {
        let mut v = s.split(",").filter_map(|s| s.trim().parse::<i32>().ok());

        let x = v.next().ok_or(format_err!("No x coordinate"))?;
        let y = v.next().ok_or(format_err!("No y coordinate"))?;

        Ok(Coordinates {
            id: None,
            x: x,
            y: y,
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn day06_2018_original_example() {
        let input = r#"1, 1
                1, 6
                8, 3
                3, 4
                5, 5
                8, 9"#;

        let coordinates = parse(&input);

        assert_eq!(coordinates.len(), 6);

        assert_eq!(coordinates[0].x, 1);
        assert_eq!(coordinates[0].y, 1);

        assert_eq!(coordinates[5].x, 8);
        assert_eq!(coordinates[5].y, 9);
    }
}
