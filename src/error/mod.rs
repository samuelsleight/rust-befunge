use std::fmt::{Show, Formatter, Result};

#[deriving(Copy, Clone)]
pub enum ParserError {
    FileReadError(String),
    UnexpectedChar(int, int, char)
}

impl Show for ParserError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            &FileReadError(ref file) => write!(f, "Error: Error reading file: {}", file),
            &UnexpectedChar(x, y, c) => write!(f, "Error: Unxpected char at ({}, {}): {}", x, y, c)
        }
    }
}
