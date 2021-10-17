use std::fmt;

#[derive(Debug)]
pub enum Error {
    Series(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Series(ref err) => write!(f, "{}", err.as_str()),
        }
    }
}
