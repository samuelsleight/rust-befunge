use std::fmt::{Show, Formatter, Result};

#[deriving(Copy, Clone)]
pub enum ParserError {
    FileReadError(String),
    FileEmptyError(String),
    UnexpectedChar(int, int, char)
}

impl Show for ParserError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            &FileReadError(ref file) => write!(f, "Error reading file: {}", file),
            &FileEmptyError(ref file) => write!(f, "File is empty: {}", file),
            &UnexpectedChar(x, y, c) => write!(f, "Unexpected char at ({}, {}): {}", x, y, c)
        }
    }
}
