use crate::{Solution, error::Result};

pub fn run(input: &str) -> Result<Solution> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offical_example_1() {
        let input = r"";

        let result = run(input).unwrap();

        assert_eq!(result.part_one, "");
        assert_eq!(result.part_two, "");
    }

    #[test]
    fn test_offical_input() {
        let input = include_str!("./input/day01");

        let solution = run(&input).unwrap();

        assert_eq!(solution.part_one, "");
        assert_eq!(solution.part_two, "");
    }
}
