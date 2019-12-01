use crate::error;
use crate::Solution;

pub fn run(input: &str) -> error::AoCResult<Solution> {
    let parsed_input = parse(input);

    Ok(Solution {
        part_one: part_one(&parsed_input).to_string(),
        part_two: String::new(),
    })
}

type Modules = Vec<f64>;

fn parse(input: &str) -> Modules {
    input
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect()
}

fn part_one(modules: &Modules) -> f64 {
    modules
        .iter()
        .map(|module| (module / 3.0).floor() - 2.0)
        .sum()
}

fn part_two(modules: &Modules) {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_offical_result() {
        let input = include_str!("./input/day01");

        let result = run(&input).unwrap();

        assert_eq!(result.part_one, "3297626");
        assert_eq!(result.part_two, "");
    }

    #[test]
    fn original_examples() {
        let input = r#"
        12
        14
        1969
        100756
        "#;

        let parsed_input = parse(input);

        let part_one_solution = part_one(&parsed_input);

        assert_eq!(part_one_solution, 34241 as f64);
    }
}
