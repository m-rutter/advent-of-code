use failure::Error;
use std::process;

use crate::{AoCError, AoCSolution};

/// Compute the solution to day 7 of AoC 2017
pub fn run(input: &str) -> Result<AoCSolution, AoCError> {
    let _input = parser(&input).unwrap_or_else(|err| {
        eprintln!("Problem parsing input: {}", err);
        process::exit(1);
    });

    Ok(AoCSolution {
        part_one: String::new(),
        part_two: String::new(),
    })
}

fn parser(input: &str) -> Result<Vec<u32>, Error> {
    let v: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();

    Ok(v)
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
