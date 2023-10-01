use std::error;
use std::fmt;

/// The error type for anomaly detection.
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// A series error.
    Series(String),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Series(ref err) => write!(f, "{}", err.as_str()),
        }
    }
}
