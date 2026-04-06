use core::error;
use core::fmt;

/// The error type for anomaly detection.
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// A series error.
    Series(&'static str),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Series(err) => f.write_str(err),
        }
    }
}
