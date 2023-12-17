use anyhow::anyhow;

use crate::{error::Result, Solution};

pub fn run(input: &str) -> Result<Solution> {
    let races = parse(input, false)?;

    let part_one: u64 = races
        .iter()
        .map(|race| race.count_solutions())
        .fold(1, |acc, count| acc * count);

    let races = parse(input, true)?;

    let part_two: u64 = races.iter().map(|race| race.count_solutions()).sum();

    Ok(Solution {
        part_one: part_one.to_string(),
        part_two: part_two.to_string(),
    })
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance_record: u64,
}

impl Race {
    fn count_solutions(&self) -> u64 {
        let mut count = 0;

        for charge in 0..self.time {
            let remaining = self.time - charge;

            let distance = charge * remaining;

            if distance > self.distance_record {
                count += 1;
            }
        }

        count
    }
}

fn parse(s: &str, part_two: bool) -> Result<Vec<Race>> {
    let lines: Vec<&str> = s.trim().lines().map(|line| line.trim()).collect();

    if lines.len() != 2 {
        Err(anyhow!(
            "Expected time and distance rows, got {} rows instead",
            lines.len()
        ))?;
    }

    let time = if part_two {
        vec![get_num_values_part_two(lines[0])?]
    } else {
        get_num_values(lines[0])?
    };
    let distance = if part_two {
        vec![get_num_values_part_two(lines[1])?]
    } else {
        get_num_values(lines[1])?
    };

    Ok(time
        .into_iter()
        .zip(distance.into_iter())
        .map(|(time, distance)| Race {
            time,
            distance_record: distance,
        })
        .collect::<Vec<Race>>())
}

fn get_num_values(s: &str) -> Result<Vec<u64>> {
    s.split(':')
        .last()
        .ok_or(anyhow!("expected time entry"))?
        .split_whitespace()
        .map(|s| s.parse::<u64>().map_err(|err| err.into()))
        .collect::<Result<Vec<u64>>>()
}

fn get_num_values_part_two(s: &str) -> Result<u64> {
    s.split(':')
        .last()
        .ok_or(anyhow!("expected time entry"))?
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .map_err(|err| err.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_examples() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "288");
        assert_eq!(solution.part_two, "71503");
    }

    #[test]
    fn match_offical_input() {
        let input = include_str!("./input/day06");

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "219849");
        assert_eq!(solution.part_two, "29432455");
    }
}
