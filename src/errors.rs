use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FileParseError {
    pub message: String,
}

impl FileParseError {
    pub fn new(msg: String) -> FileParseError {
        FileParseError { message: msg }
    }
}

impl fmt::Display for FileParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing file: {}", self.message)
    }
}

impl Error for FileParseError {}

#[derive(Debug, Clone)]
pub struct StringParseError {
    pub message: String,
}

impl StringParseError {
    pub fn new(msg: String) -> StringParseError {
        StringParseError { message: msg }
    }
}

impl fmt::Display for StringParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing string: {}", self.message)
    }
}

impl Error for StringParseError {}
