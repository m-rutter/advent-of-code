use crate::{error, AoCSolution};

/// Compute the solution to day 7 of AoC 2017
pub fn run(input: &str) -> error::AoCResult<AoCSolution> {
    let _input = parser(&input);

    unimplemented!()
}

fn parser(input: &str) -> Vec<u32> {
    input.chars().filter_map(|c| c.to_digit(10)).collect()
}

#[cfg(test)]
mod tests {
    // TODO: Solve test and put the offical answer here
    // use super::*;

    // #[test]
    // fn matches_offical_result() {
    //     let input = include_str!("./input/day07");

    //     let config = Config {
    //         day: 1,
    //         input: input.to_string(),
    //     };

    //     let _result = run(&config.input).unwrap();
    // }

}
