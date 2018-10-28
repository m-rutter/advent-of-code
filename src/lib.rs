//! Advent of Code (AoC) solutions library written in Rust.
//! Personal learning project.
use failure::Fail;

mod aoc2017;

/// AoC config
#[derive(Debug)]
pub struct Config {
    year: u16,
    day: u8,
    input: String,
}

/// Solution for a day in AoC
pub struct AoCSolution {
    pub part_one: String,
    pub part_two: String,
}

/// AoC Error type
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
    /// use advent_of_code::Config;
    ///
    /// let config = Config::new(2017, 1, String::from("6497139596"));
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
/// use advent_of_code::{solve_day, Config};
///
/// let mut config = Config::new(2017, 1, String::from("91212129"));
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
            1 => aoc2017::day01::run(&config.input),
            2 => aoc2017::day02::run(&config.input),
            3 => aoc2017::day03::run(&config.input),
            4 => aoc2017::day04::run(&config.input),
            5 => aoc2017::day05::run(&config.input),
            6 => aoc2017::day06::run(&config.input),
            7 => aoc2017::day07::run(&config.input),
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
