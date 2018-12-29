use crate::{error, Solution};
use std::collections::HashSet;
use std::ops::Range;
use std::str::FromStr;

pub fn run(input: &str) -> error::AoCResult<Solution> {
    let coordinates = parse(&input);

    if coordinates.is_empty() {
        Err(error::Error::msg(
            &"Insufficent coordinates parsed from input",
        ))?
    }

    let upper_bound = 1 + coordinates
        .iter()
        .map(|c| if c.x > c.y { c.x } else { c.y })
        .max()
        .expect("The length of coordinates ought to be greater than 0")
        as usize;

    let range = 0..upper_bound;

    let grid = compute_partial_grid(&coordinates, &range);

    let coordinates_to_ignore = find_coordinates_to_ignore(&grid, &range);

    let mut regions = vec![0; coordinates.len()];
    for x in range.clone() {
        for y in range.clone() {
            if let Some(coordinates) = grid[x][y] {
                if !coordinates_to_ignore.contains(&coordinates) {
                    regions[coordinates] += 1;
                }
            }
        }
    }

    let largest_area_size = regions
        .iter()
        .max_by(|a, b| a.cmp(&b))
        .expect("There should be at least one region by this point");

    let mut optimial_region_size = 0;
    for x in range.clone() {
        for y in range.clone() {
            let sum: i32 = coordinates
                .iter()
                .map(|c| c.distince_from((x as i32, y as i32)))
                .sum();

            if sum < 10000 {
                optimial_region_size += 1;
            }
        }
    }

    Ok(Solution {
        part_one: largest_area_size.to_string(),
        part_two: optimial_region_size.to_string(),
    })
}

type Grid = Vec<Vec<Option<usize>>>;

fn compute_partial_grid(c: &[Coordinates], range: &Range<usize>) -> Grid {
    let mut grid = vec![vec![None; range.end]; range.end];

    for x in range.clone() {
        for y in range.clone() {
            let distances: Vec<(usize, i32)> = c
                .iter()
                .enumerate()
                .map(|(i, c)| (i, c.distince_from((x as i32, y as i32))))
                .collect();

            let shortest = distances.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();

            let closest: Vec<&(usize, i32)> =
                distances.iter().filter(|(_, c)| *c == shortest.1).collect();

            match closest.len() {
                1 => grid[x][y] = Some(shortest.0),
                _ => grid[x][y] = None,
            };
        }
    }

    grid
}

#[allow(clippy::ptr_arg)]
fn find_coordinates_to_ignore(grid: &Grid, range: &Range<usize>) -> HashSet<usize> {
    range.clone().fold(HashSet::new(), |mut acc, i| {
        if let Some(c) = grid[0][i] {
            acc.insert(c);
        }
        if let Some(c) = grid[i][0] {
            acc.insert(c);
        }
        if let Some(c) = grid[range.end - 1][i] {
            acc.insert(c);
        }
        if let Some(c) = grid[i][range.end - 1] {
            acc.insert(c);
        }

        acc
    })
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
        (other.0 - self.x).abs() + (other.1 - self.y).abs()
    }
}

impl FromStr for Coordinates {
    type Err = error::Error;

    fn from_str(s: &str) -> error::AoCResult<Coordinates> {
        let mut v = s.split(',').filter_map(|s| s.trim().parse::<i32>().ok());

        let x = v
            .next()
            .ok_or_else(|| error::Error::msg(&"No x coordinate"))?;
        let y = v
            .next()
            .ok_or_else(|| error::Error::msg(&"No y coordinate"))?;

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

        let r = run(&input).unwrap();

        assert_eq!(r.part_one, "17");
    }
}
