use std::error;
use std::num;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    Malformed(String),
    InvalidInt(num::ParseIntError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::Malformed(ref str) => write!(f, "Malformed {}!", str),
            ParseError::InvalidInt(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ParseError::Malformed(_) => None,
            ParseError::InvalidInt(ref err) => Some(err),
        }
    }
}

impl From<num::ParseIntError> for ParseError {
    fn from(err: num::ParseIntError) -> ParseError {
        ParseError::InvalidInt(err)
    }
}

impl From<&str> for ParseError {
    fn from(err: &str) -> ParseError {
        ParseError::Malformed(err.into())
    }
}

impl From<String> for ParseError {
    fn from(err: String) -> ParseError {
        ParseError::Malformed(err)
    }
}