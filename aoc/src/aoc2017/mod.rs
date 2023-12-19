//! AoC 2017 module
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

use crate::{error, Config, Solution};

pub fn run(config: &Config) -> error::Result<Solution> {
    match config.day {
        1 => day01::run(&config.input),
        2 => day02::run(&config.input),
        3 => day03::run(&config.input),
        4 => day04::run(&config.input),
        5 => day05::run(&config.input),
        6 => day06::run(&config.input),
        7 => day07::run(&config.input),
        _ => Err(error::AoCError::UnsupportedDay {
            day: config.day,
            year: config.year,
        }),
    }
}
