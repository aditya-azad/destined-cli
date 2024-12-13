use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FileParseError {
    message: String,
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
