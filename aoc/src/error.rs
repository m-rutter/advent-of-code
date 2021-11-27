//! AoC error module
use intcode::error::IntCodeError;
use std::error::Error as StdError;
use std::fmt;

// Convenience Result type
pub type AoCResult<T> = std::result::Result<T, Error>;

/// An error type for the Advent of Code crate
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    source: Option<Box<dyn StdError + Send + Sync + 'static>>,
}

impl Error {
    /// Returns the error kind
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    /// Creates generic error with a message
    pub(crate) fn msg(value: &impl ToString) -> Self {
        Self {
            kind: ErrorKind::Msg(value.to_string()),
            source: None,
        }
    }

    #[allow(dead_code)]
    ///Creates generic error with a message and a cause
    pub(crate) fn chain(
        value: &impl ToString,
        cause: impl StdError + Send + Sync + 'static,
    ) -> Self {
        Self {
            kind: ErrorKind::Msg(value.to_string()),
            source: Some(cause.into()),
        }
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&(dyn StdError)> {
        self.source().as_ref().map(|c| &**c)
    }

    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_ref()
            .map(|c| c.as_ref() as &(dyn StdError + 'static))
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Generic error message
    Msg(String),
    /// Error when parsing provided input
    InputParse,
    /// Error when the day is not supported or does not exist
    UnsupportedDay { year: u16, day: u8 },
}

impl From<ErrorKind> for Error {
    fn from(error: ErrorKind) -> Self {
        Self {
            kind: error,
            source: None,
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Self {
            kind: ErrorKind::InputParse,
            source: Some(error.into()),
        }
    }
}

impl From<chrono::format::ParseError> for Error {
    fn from(error: chrono::format::ParseError) -> Self {
        Self {
            kind: ErrorKind::InputParse,
            source: Some(error.into()),
        }
    }
}

impl<T> From<pest::error::Error<T>> for Error {
    fn from(_: pest::error::Error<T>) -> Self {
        Self {
            kind: ErrorKind::InputParse,
            source: None,
        }
    }
}

impl From<IntCodeError> for Error {
    fn from(error: IntCodeError) -> Self {
        Self {
            kind: ErrorKind::Msg(format!("IntCode Execution Error")),
            source: Some(error.into()),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::Msg(message) => write!(f, "{}", message),
            ErrorKind::InputParse => write!(f, "Error parsing input"),
            ErrorKind::UnsupportedDay { day, year } => write!(
                f,
                "Day {} for year {} either does not exist or is unsupported",
                day, year
            ),
        }
    }
}
