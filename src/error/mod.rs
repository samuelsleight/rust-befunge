use std::fmt::{Show, Formatter, Result};

#[deriving(Copy, Clone)]
pub enum ParserError {
    FileReadError(String),
    FileEmptyError(String),
    UnexpectedChar(int, int, char),
    OutputError
}

impl Show for ParserError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            &FileReadError(ref file) => write!(f, "Unable to read file: {}", file),
            &FileEmptyError(ref file) => write!(f, "File is empty: {}", file),
            &UnexpectedChar(x, y, c) => write!(f, "Unexpected char at ({}, {}): {}", x, y, c),
            &OutputError => write!(f, "Unable to write output")
        }
    }
}
