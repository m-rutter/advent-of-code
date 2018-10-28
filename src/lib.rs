//! Advent of Code (AoC) 2017 solutions library written in Rust.
//! Personal learning project.
use failure::Fail;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

/// AoC config
#[derive(Debug)]
pub struct Config {
    day: u8,
    input: String,
}

/// Solution for a day in AoC 2017
pub struct AoCSolution {
    pub part_one: String,
    pub part_two: String,
}

/// Aoc Error type
#[derive(Debug, Fail)]
pub enum AoCError {
    #[fail(display = "Invalid input for day {}: {}", day, input)]
    InvalidInput { day: u8, input: String },

    #[fail(display = "Day {} is not implemented", day)]
    InvalidDay { day: u8 },
}

impl Config {
    /// Creates a instance of AoC Config defaulting to using stdin
    /// # Example
    /// ```
    /// use aoc_2017::Config;
    ///
    /// let config = Config::new(1, String::from("6497139596"));
    /// ```
    pub fn new(day: u8, input: String) -> Config {
        Config {
            day: day,
            input: input,
        }
    }
}

/// Computes the solution to a day in Advent of Code 2017
/// # Examples
/// ```rust
/// use aoc_2017::{solve_day, Config};
///
/// let mut config = Config::new(1, String::from("6497139596"));
///  
/// match solve_day(&config) {
///     Ok(solution) => {
///         println!("Part 1 solution for day 1: {}", solution.part_one);
///         println!("Part 2 solution for day 1: {}", solution.part_two);
///     },
///     Err(error) => {
///         eprintln!("{}", error)
///     }
/// };
///
/// ```
pub fn solve_day(config: &Config) -> Result<AoCSolution, AoCError> {
    match config.day {
        1 => day01::run(&config),
        2 => day02::run(&config),
        3 => day03::run(&config),
        4 => day04::run(&config),
        5 => day05::run(&config),
        6 => day06::run(&config),
        7 => day07::run(&config),
        _ => Err(AoCError::InvalidDay { day: config.day }),
    }
}
