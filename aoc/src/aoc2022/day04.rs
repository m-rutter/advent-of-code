use crate::{error::Result, Solution};

pub fn run(input: &str) -> Result<Solution> {
    let mut part_one = 0;
    let mut part_two = 0;

    for line in input.trim().lines() {
        let (pair1, pair2) = line
            .trim()
            .split_once(',')
            .ok_or_else(|| anyhow::anyhow!("Expected pair delimited by ','"))?;

        let (start_a, end_a) = pair1
            .split_once('-')
            .ok_or_else(|| anyhow::anyhow!("Expected pair delimited by '-'"))?;

        let (start_b, end_b) = pair2
            .split_once('-')
            .ok_or_else(|| anyhow::anyhow!("Expected pair delimited by '-'"))?;

        let start_a: u32 = start_a.parse()?;
        let start_b: u32 = start_b.parse()?;
        let end_a: u32 = end_a.parse()?;
        let end_b: u32 = end_b.parse()?;

        if start_a <= start_b && end_a >= end_b {
            part_one += 1;
        } else if start_b <= start_a && end_b >= end_a {
            part_one += 1;
        }

        if end_a >= start_b && start_a <= end_b {
            part_two += 1;
        }
    }

    Ok(Solution {
        part_one: part_one.to_string(),
        part_two: part_two.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_examples() {
        let example = r"
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        ";

        let solution = run(example).unwrap();

        assert_eq!(solution.part_one, "2");
        assert_eq!(solution.part_two, "4");
    }

    #[test]
    fn matches_offical_results() {
        let input = include_str!("./input/day04");

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "498");
        assert_eq!(solution.part_two, "859");
    }
}
