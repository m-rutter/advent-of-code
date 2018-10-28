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

pub use self::day01::run as day01_run;
pub use self::day02::run as day02_run;
pub use self::day03::run as day03_run;
pub use self::day04::run as day04_run;
pub use self::day05::run as day05_run;
pub use self::day06::run as day06_run;
pub use self::day07::run as day07_run;

/// AoC config
#[derive(Debug)]
pub struct Config {
    year: u16,
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
    /// Error when something is wrong with the input data when being parsed
    #[fail(display = "Invalid input for day {} of year {}: {}", day, year, input)]
    InvalidInput { day: u8, year: u16, input: String },

    /// Error when the day is not implemented or does not exist
    #[fail(display = "Day {} of year {} is not implemented", day, year)]
    InvalidDay { year: u16, day: u8 },
}

impl Config {
    /// Creates a instance of AoC Config defaulting to using stdin
    /// # Example
    /// ```
    /// use aoc_2017::Config;
    ///
    /// let config = Config::new(1, String::from("6497139596"));
    /// ```
    pub fn new(year: u16, day: u8, input: String) -> Config {
        Config {
            year: year,
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
/// let mut config = Config::new(1, String::from("91212129"));
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
    match config.year {
        2017 => match config.day {
            1 => day01::run(&config.input),
            2 => day02::run(&config.input),
            3 => day03::run(&config.input),
            4 => day04::run(&config.input),
            5 => day05::run(&config.input),
            6 => day06::run(&config.input),
            7 => day07::run(&config.input),
            _ => Err(AoCError::InvalidDay {
                year: 2017,
                day: config.day,
            }),
        },
        _ => Err(AoCError::InvalidDay {
            year: config.year,
            day: config.day,
        }),
    }
}
