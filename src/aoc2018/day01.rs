use crate::error;
use crate::AoCSolution;
use std::collections::HashSet;

pub fn run(input: &str) -> error::AoCResult<AoCSolution> {
    let modulations = parse(&input);

    Ok(AoCSolution {
        part_one: fold_frequency_modulations(&modulations).to_string(),
        part_two: find_repeating_frequency(&modulations).to_string(),
    })
}

fn fold_frequency_modulations(modulations: &[i32]) -> i32 {
    modulations.iter().sum()
}

fn find_repeating_frequency(modulations: &[i32]) -> i32 {
    let mut frequencies: HashSet<i32> = HashSet::new();
    let mut current_frequency: i32 = 0;

    frequencies.insert(current_frequency);

    for modulation in modulations.iter().cycle().take(1_000_000) {
        current_frequency += modulation;

        if frequencies.contains(&current_frequency) {
            break;
        }

        frequencies.insert(current_frequency);
    }

    current_frequency
}

fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn matches_offical_result() {
        let input = include_str!("./input/day01");

        let result = run(&input).unwrap();

        assert_eq!(result.part_one, "505");
        assert_eq!(result.part_two, "72330");
    }

}
