use crate::error;
use crate::Solution;

pub fn run(input: &str) -> error::AoCResult<Solution> {
    let registers = parse(input);

    unimplemented!();
}

fn parse(input: &str) -> Vec<u32> {
    input
        .split(',')
        .filter_map(|code| code.trim().parse().ok())
        .collect()
}

enum Op {
    Add(Positions),
    Multiply(Positions),
    Terminal,
}

struct Positions {
    param1: u32,
    param2: u32,
    output: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_examples() {
        let input = "";
    }

    #[test]
    fn matches_offical_result() {
        let input = include_str!("./input/day02");

        let result = run(&input).unwrap();
    }
}
