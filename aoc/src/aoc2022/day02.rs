use crate::{error::Result, Solution};

pub fn run(_input: &str) -> Result<Solution> {
    todo!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn matches_examples() {
        let example = r"";

        let _solution = run(example).unwrap();
    }

    #[test]
    fn matches_offical_results() {
        let input = include_str!("./input/day02");

        let _solution = run(input).unwrap();
    }
}
