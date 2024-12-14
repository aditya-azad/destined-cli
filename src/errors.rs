use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParsingError {
    File(String),
    String(String),
}

impl Error for ParsingError {}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParsingError::File(msg) => write!(f, "Error parsing file: {}", msg),
            ParsingError::String(msg) => write!(f, "Error parsing string: {}", msg),
        }
    }
}
