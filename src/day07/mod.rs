use failure::Error;
use std::process;

use super::{AoCError, AoCSolution, Config};

pub fn run(config: &Config) -> Result<AoCSolution, AoCError> {
    let _input = parser(&config.input).unwrap_or_else(|err| {
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
    use super::*;

    // TODO: Solve test and put the offical answer here
    // #[test]
    // fn matches_offical_result() {
    //     let input = include_str!("./input");

    //     let config = Config {
    //         day: 1,
    //         input: input.to_string(),
    //     };

    //     let _result = run(&config).unwrap();
    // }

}
