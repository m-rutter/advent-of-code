//! Advent of code error module
use std::convert::Into;
use std::error::Error as StdError;
use std::fmt;

// Convenience Result type
pub type AoCResult<T> = std::result::Result<T, Error>;

/// An error type for the Advent of Code crate
#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    source: Option<Box<dyn StdError>>,
}

impl Error {
    /// Creates generic error with a message
    pub(crate) fn msg(value: impl ToString) -> Self {
        Self {
            kind: ErrorKind::Msg(value.to_string()),
            source: None,
        }
    }

    ///Creates generic error with a message and a cause
    pub(crate) fn chain(
        value: impl ToString,
        cause: impl Into<Box<dyn StdError + Send + Sync + 'static>>,
    ) -> Self {
        Self {
            kind: ErrorKind::Msg(value.to_string()),
            source: Some(cause.into()),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_ref().map(|c| &**c)
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Msg(String),
    InputParse,
    UnsupportedDay {
        year: u16,
        day: u8,
    },
    /// Hint to users of the crate that this ErrorKind ought to be matched as
    /// non-exhaustive in case the enum grows
    #[doc(hidden)]
    __Nonexhaustive,
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::Msg(message) => write!(f, "{}", message),
            ErrorKind::InputParse => write!(f, "Error parsing input"),
            ErrorKind::UnsupportedDay { year, day } => write!(
                f,
                "Day {} for year {} either does not exist or is unsupported",
                day, year
            ),
            ErrorKind::__Nonexhaustive => write!(f, "Nonexhaustive"),
        }
    }
}
