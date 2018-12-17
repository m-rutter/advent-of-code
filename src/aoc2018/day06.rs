use crate::{error, AoCSolution};
use failure::{format_err, Error};
use std::collections::HashSet;
use std::str::FromStr;

pub fn run(input: &str) -> error::AoCResult<AoCSolution> {
    let coordinates = parse(&input);

    let upper_bound = 1 + coordinates
        .iter()
        .map(|c| if c.x > c.y { c.x } else { c.y })
        .max()
        .unwrap() as usize;

    let mut grid = vec![vec![0 as usize; upper_bound]; upper_bound];

    let range = 0..upper_bound;

    for x in range.clone() {
        for y in range.clone() {
            let distances: Vec<(usize, i32)> = coordinates
                .iter()
                .enumerate()
                .map(|(i, c)| (i, c.distince_from((x as i32, y as i32))))
                .collect();

            let shortest = distances.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();

            grid[x][y] = shortest.0;
        }
    }

    let coordinates_to_ignore: HashSet<(usize, usize)> =
        range.clone().fold(HashSet::new(), |mut acc, i| {
            acc.insert((0, i));
            acc.insert((i, 0));
            acc.insert((upper_bound - 1, i));
            acc.insert((i, upper_bound - 1));

            acc
        });

    let mut regions = vec![0; coordinates.len()];
    for x in range.clone() {
        for y in range.clone() {
            let point = grid[x][y];

            if !coordinates_to_ignore.contains(&(x, y)) {
                regions[point] += 1
            }
        }
    }

    let largest_area = regions
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .1;

    println!("{}", largest_area);

    unimplemented!()
}

fn parse(input: &str) -> Vec<Coordinates> {
    input
        .lines()
        .filter_map(|line| Coordinates::from_str(&line).ok())
        .collect()
}

struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn distince_from(&self, other: (i32, i32)) -> i32 {
        (self.x - other.0).abs() + (self.y - other.0).abs()
    }
}

impl FromStr for Coordinates {
    type Err = Error;

    fn from_str(s: &str) -> Result<Coordinates, Error> {
        let mut v = s.split(',').filter_map(|s| s.trim().parse::<i32>().ok());

        let x = v.next().ok_or_else(|| format_err!("No x coordinate"))?;
        let y = v.next().ok_or_else(|| format_err!("No y coordinate"))?;

        Ok(Coordinates { x: x, y: y })
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
