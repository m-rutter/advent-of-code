//! AoC error module

use intcode::error::Error as IntCodeError;

use thiserror::Error;

/// Convenience Result type
pub type Result<T> = std::result::Result<T, AoCError>;

/// AoC Error type
#[derive(Error, Debug)]
pub enum AoCError {
    /// Generic error
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    /// Error when parsing provided input
    #[error("test input parsing error")]
    ParseError(#[from] ParsingError),
    /// Error when the day is not supported or does not exist
    #[error("unsupported year (expected {year:?} and/or day {day:?})")]
    UnsupportedDay { year: u16, day: u8 },
    /// An error coming from intcode execution
    #[error("an IntCode Execution Error")]
    IntCodeError(#[from] IntCodeError),
}

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error(transparent)]
    ChronoParseError(#[from] chrono::format::ParseError),

    #[error(transparent)]
    IntParseError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    RegexParseError(#[from] regex::Error),

    #[error("test input parsing error")]
    ParseError,
}

macro_rules! impl_into_aoc_error {
    ($ty: ty) => {
        impl From<$ty> for AoCError {
            fn from(error: $ty) -> Self {
                error.into()
            }
        }
    };
}

impl_into_aoc_error!(chrono::format::ParseError);
impl_into_aoc_error!(std::num::ParseIntError);
