//! Advent of code error module
use failure::{Backtrace, Context, Fail};
use std::fmt;
use std::num::ParseIntError;

pub type AoCResult<T> = std::result::Result<T, AoCError>;

/// Aoc Error Type
#[derive(Debug)]
pub struct AoCError {
    inner: Context<AoCErrorKind>,
}

impl AoCError {
    pub fn kind(&self) -> AoCErrorKind {
        *self.inner.get_context()
    }
}

impl From<AoCErrorKind> for AoCError {
    fn from(kind: AoCErrorKind) -> AoCError {
        AoCError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<AoCErrorKind>> for AoCError {
    fn from(inner: Context<AoCErrorKind>) -> AoCError {
        AoCError { inner: inner }
    }
}

impl From<ParseIntError> for AoCError {
    fn from(_err: ParseIntError) -> AoCError {
        AoCError::from(AoCErrorKind::InvalidInput {
            message: "expected unsigned interger",
        })
    }
}

impl Fail for AoCError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for AoCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

/// AoC Error Kind
#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum AoCErrorKind {
    /// Error when something is wrong with the input data when being parsed
    #[fail(display = "Invalid input: {}", message)]
    InvalidInput { message: &'static str },
    /// Error when the day is not implemented or does not exist
    #[fail(display = "Day is not yet implemented or does not exist")]
    InvalidDay,
}
