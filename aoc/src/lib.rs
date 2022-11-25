//! Advent of Code (AoC) solutions library written in Rust.
//! Personal learning project. Very much incomplete.
//! # Example
//! ```rust
//! use advent_of_code::{solve_day, Config};
//!
//! let config = Config::new(2017, 1, String::from("91212129"));
//!
//! match solve_day(&config) {
//!     Ok(solution) => {
//!         assert_eq!(solution.part_one, "9");
//!         assert_eq!(solution.part_two, "6");
//!     },
//!     Err(error) => {
//!         //...
//!     }
//! };
//!
//! ```
pub mod aoc2017;
pub mod aoc2018;
pub mod aoc2019;
pub mod aoc2020;
pub mod aoc2021;
pub mod error;

/// AoC config
#[derive(Debug)]
pub struct Config {
    pub year: u16,
    pub day: u8,
    pub input: String,
}

/// Solution for a day in AoC
#[derive(Debug)]
pub struct Solution {
    /// Answer to part one of challenge
    pub part_one: String,
    /// Answer to part two of challenge
    pub part_two: String,
}

impl Config {
    /// Creates a instance of AoC Config
    /// # Example
    /// ```
    /// use advent_of_code::Config;
    ///
    /// let config = Config::new(2017, 1, String::from("6497139596"));
    /// ```
    pub fn new(year: u16, day: u8, input: String) -> Self {
        Config { year, day, input }
    }
}

/// Computes the solution to a day in Advent of Code 2017/2018/2019
/// # Example
/// ```rust
/// use advent_of_code::{solve_day, Config};
///
/// let mut config = Config::new(2017, 1, String::from("91212129"));
///
/// match solve_day(&config) {
///     Ok(solution) => {
///         assert_eq!(solution.part_one, "9");
///         assert_eq!(solution.part_two, "6");
///     },
///     Err(error) => {
///         //...
///     }
/// };
///
/// ```
pub fn solve_day(config: &Config) -> error::Result<Solution> {
    match config.year {
        2017 => match config.day {
            1 => aoc2017::day01::run(&config.input),
            2 => aoc2017::day02::run(&config.input),
            3 => aoc2017::day03::run(&config.input),
            4 => aoc2017::day04::run(&config.input),
            5 => aoc2017::day05::run(&config.input),
            6 => aoc2017::day06::run(&config.input),
            7 => aoc2017::day07::run(&config.input),
            _ => Err(error::ErrorKind::UnsupportedDay {
                day: config.day,
                year: config.year,
            })?,
        },
        2018 => match config.day {
            1 => aoc2018::day01::run(&config.input),
            2 => aoc2018::day02::run(&config.input),
            3 => aoc2018::day03::run(&config.input),
            4 => aoc2018::day04::run(&config.input),
            5 => aoc2018::day05::run(&config.input),
            6 => aoc2018::day06::run(&config.input),
            7 => aoc2018::day07::run(&config.input),
            _ => Err(error::ErrorKind::UnsupportedDay {
                day: config.day,
                year: config.year,
            })?,
        },
        2019 => match config.day {
            1 => aoc2019::day01::run(&config.input),
            2 => aoc2019::day02::run(&config.input),
            _ => Err(error::ErrorKind::UnsupportedDay {
                day: config.day,
                year: config.year,
            })?,
        },
        2020 => match config.day {
            1 => aoc2020::day01::run(&config.input),
            2 => aoc2020::day02::run(&config.input),
            _ => Err(error::ErrorKind::UnsupportedDay {
                day: config.day,
                year: config.year,
            })?,
        },
        2021 => match config.day {
            1 => aoc2021::day01::run(&config.input),
            2 => aoc2021::day02::run(&config.input),
            3 => aoc2021::day03::run(&config.input),
            _ => Err(error::ErrorKind::UnsupportedDay {
                day: config.day,
                year: config.year,
            })?,
        },
        _ => Err(error::ErrorKind::UnsupportedDay {
            day: config.day,
            year: config.year,
        })?,
    }
}
