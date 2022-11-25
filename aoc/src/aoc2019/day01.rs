use crate::error;
use crate::Solution;

pub fn run(input: &str) -> error::Result<Solution> {
    let parsed_input = parse(input);

    Ok(Solution {
        part_one: part_one(&parsed_input).iter().sum::<Module>().to_string(),
        part_two: part_two(&parsed_input).to_string(),
    })
}

type Module = i64;

fn parse(input: &str) -> Vec<Module> {
    input
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect()
}

fn fuel_calc(module: &Module) -> Module {
    module / 3 - 2
}

fn part_one(modules: &[Module]) -> Vec<Module> {
    modules.iter().map(fuel_calc).collect()
}

fn part_two(modules: &[Module]) -> Module {
    part_one(modules)
        .iter()
        .map(|fuel_mass| {
            let mut fuel = *fuel_mass;
            let mut fuel_to_add = *fuel_mass;
            loop {
                fuel_to_add = fuel_calc(&fuel_to_add);
                if fuel_to_add <= 0 {
                    break fuel;
                }
                fuel += fuel_to_add;
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_offical_result() {
        let input = include_str!("./input/day01");

        let result = run(&input).unwrap();

        assert_eq!(result.part_one, "3297626");
        assert_eq!(result.part_two, "4943578");
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
        let part_two_solution = part_two(&[100756]);

        assert_eq!(part_one_solution.iter().sum::<Module>(), 34241);
        assert_eq!(part_two_solution, 50346);
    }
}
