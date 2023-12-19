pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;

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
        8 => day08::run(&config.input),
        9 => day09::run(&config.input),
        10 => day10::run(&config.input),
        _ => Err(error::AoCError::UnsupportedDay {
            day: config.day,
            year: config.year,
        }),
    }
}
