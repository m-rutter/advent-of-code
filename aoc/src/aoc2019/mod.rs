//! AoC 2019 module
pub mod day01;
pub mod day02;

use crate::{error, Config, Solution};

pub fn run(config: &Config) -> error::Result<Solution> {
    match config.day {
        1 => day01::run(&config.input),
        2 => day02::run(&config.input),
        _ => Err(error::AoCError::UnsupportedDay {
            day: config.day,
            year: config.year,
        }),
    }
}
