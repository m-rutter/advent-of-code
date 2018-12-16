//! Advent of Code (AoC) solutions library written in Rust.
//! Personal learning project.

pub mod aoc2017;
pub mod aoc2018;
pub mod error;

/// AoC config
#[derive(Debug)]
pub struct Config {
    year: u16,
    day: u8,
    input: String,
}

/// Solution for a day in AoC
pub struct AoCSolution {
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
pub fn solve_day(config: &Config) -> error::AoCResult<AoCSolution> {
    match config.year {
        2017 => match config.day {
            1 => aoc2017::day01::run(&config.input),
            2 => aoc2017::day02::run(&config.input),
            3 => aoc2017::day03::run(&config.input),
            4 => aoc2017::day04::run(&config.input),
            5 => aoc2017::day05::run(&config.input),
            6 => aoc2017::day06::run(&config.input),
            7 => aoc2017::day07::run(&config.input),
            _ => Err(error::AoCErrorKind::InvalidDay)?,
        },
        2018 => match config.day {
            1 => aoc2018::day01::run(&config.input),
            2 => aoc2018::day02::run(&config.input),
            3 => aoc2018::day03::run(&config.input),
            4 => aoc2018::day04::run(&config.input),
            5 => aoc2018::day05::run(&config.input),
            _ => Err(error::AoCErrorKind::InvalidDay)?,
        },
        _ => Err(error::AoCErrorKind::InvalidDay)?,
    }
}
