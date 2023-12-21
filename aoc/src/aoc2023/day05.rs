use itertools::Itertools;
use std::{isize, str::FromStr};

use anyhow::anyhow;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;

use crate::{error::AoCError, error::Result, Solution};

pub fn run(input: &str) -> Result<Solution> {
    // spliting on empty new lines
    let sections: Vec<&str> = input.split("\n\n").collect();

    if sections.len() != 8 {
        Err(anyhow!("expected eight sections, got {}", sections.len()))?;
    }

    // first section is the seeds
    let seeds = get_line_numbers(sections[0]);

    // rest are maps
    let seed_soil = get_maps(sections[1])?;
    let soil_fertilizer = get_maps(sections[2])?;
    let fertilizer_water = get_maps(sections[3])?;
    let water_light = get_maps(sections[4])?;
    let light_temperature = get_maps(sections[5])?;
    let temperature_humidity = get_maps(sections[6])?;
    let humidity_location = get_maps(sections[7])?;

    let maps = [
        seed_soil,
        soil_fertilizer,
        fertilizer_water,
        water_light,
        light_temperature,
        temperature_humidity,
        humidity_location,
    ];

    let part_one = find_lowest_location(&seeds, &maps).unwrap();

    // get all of the ranges

    let part_two = find_lowest_location_seed_ranges(seeds, &maps).unwrap();

    Ok(Solution {
        part_one: part_one.to_string(),
        part_two: part_two.to_string(),
    })
}

fn find_lowest_location_seed_ranges(seeds: Vec<isize>, list_of_maps: &[Vec<Map>]) -> Option<isize> {
    seeds
        .into_iter()
        // batch entries into pairs
        .batching(|it| match it.next() {
            None => None,
            Some(x) => match it.next() {
                None => None,
                Some(y) => Some((x, y)),
            },
        })
        // compute seed ranges
        .flat_map(|(a, b)| {
            let v = (a..a + b).map(|n| n).collect_vec();
            v.par_iter()
                .map(|n| n)
                .map(|seed| {
                    let mut current_source = *seed;
                    for map_list in list_of_maps {
                        for map in map_list {
                            let dest = map.from_source(current_source);

                            if let Some(dest) = dest {
                                current_source = dest;
                                break;
                            }
                        }
                    }

                    current_source
                })
                .min()
        })
        // using rayon to parallelise the brute force search
        .min()
}

fn find_lowest_location(seeds: &[isize], list_of_maps: &[Vec<Map>]) -> Option<isize> {
    seeds
        // using rayon to parallelise the brute force search
        .par_iter()
        .map(|seed| {
            let mut current_source = *seed;
            for map_list in list_of_maps {
                for map in map_list {
                    let dest = map.from_source(current_source);

                    if let Some(dest) = dest {
                        current_source = dest;
                        break;
                    }
                }
            }

            current_source
        })
        .min()
}

#[derive(Debug, Clone)]
struct Map {
    destination_start: isize,
    source_start: isize,
    range_length: isize,
}

impl Map {
    fn from_source(&self, source: isize) -> Option<isize> {
        let source_end = self.source_start + self.range_length;

        match source {
            source if source >= self.source_start && source < source_end => {
                let destination_end = self.destination_start + self.range_length;
                let offset = source_end - source;

                Some(destination_end - offset)
            }
            _ => None,
        }
    }
}

impl FromStr for Map {
    type Err = AoCError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let nums = get_line_numbers(s);

        let len = nums.len();

        if len != 3 {
            Err(anyhow!("expected three numbers per map, got {len}"))?;
        }

        Ok(Map {
            destination_start: nums[0],
            source_start: nums[1],
            range_length: nums[2],
        })
    }
}

static NUMBERS_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?P<num>\d+)").expect("unable to compile number regex"));

fn get_line_numbers(s: &str) -> Vec<isize> {
    NUMBERS_RE
        .captures_iter(s)
        .map(|s| {
            s.name("num")
                .expect("expected num group")
                .as_str()
                .parse()
                .expect("expected numbers")
        })
        .collect()
}
fn get_maps(section: &str) -> Result<Vec<Map>> {
    section.lines().skip(1).map(|line| line.parse()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_examples() {
        let input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "35");
        assert_eq!(solution.part_two, "46");
    }

    #[ignore = "too expensive to run"]
    #[test]
    fn match_offical_input() {
        let input = include_str!("./input/day05");

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "318728750");
        assert_eq!(solution.part_two, "37384986");
    }
}
