use crate::error::AoCResult;
use crate::Solution;

pub fn run(input: &str) -> AoCResult<Solution> {
    let measurements = parse(input)?;

    let window_sums: Vec<u32> = measurements
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();

    Ok(Solution {
        part_one: calc_rate_of_increase(&measurements).to_string(),
        part_two: calc_rate_of_increase(&window_sums).to_string(),
    })
}

fn calc_rate_of_increase(measurements: &[u32]) -> u32 {
    measurements
        .iter()
        .zip(measurements.iter().skip(1))
        .filter(|(curr, next)| curr < next)
        .count() as u32
}

fn parse(input: &str) -> AoCResult<Vec<u32>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().parse::<u32>().map_err(|err| err.into()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        let input = r#"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
        "#;
        let measurements = parse(input).expect("parse failed");

        assert_eq!(
            measurements,
            vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
        );
    }

    #[test]
    fn original_examples() {
        let input = r#"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
        "#;

        let res = run(input).unwrap();

        assert_eq!(res.part_one, "7");
        assert_eq!(res.part_two, "5");
    }

    #[test]
    fn matches_offical_result() {
        let input = include_str!("./input/day01");

        let result = run(&input).unwrap();

        assert_eq!(result.part_one, "1387");
        assert_eq!(result.part_two, "1362");
    }
}
