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
pub mod aoc2022;
pub mod aoc2023;
pub mod aoc2024;
pub mod error;

/// AoC config
#[derive(Debug)]
pub struct Config {
    pub year: u16,
    pub day: u8,
    pub input: String,
}

/// Solution for a day in AoC
#[derive(Debug, Clone)]
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
        2017 => aoc2017::run(config),
        2018 => aoc2018::run(config),
        2019 => aoc2019::run(config),
        2020 => aoc2020::run(config),
        2021 => aoc2021::run(config),
        2022 => aoc2022::run(config),
        2023 => aoc2023::run(config),
        _ => Err(error::AoCError::UnsupportedDay {
            day: config.day,
            year: config.year,
        }),
    }
}
