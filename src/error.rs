use std::{
    io,
    fmt::{self, Formatter, Display}
};

use crate::interpreter;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Interpreter(interpreter::Error)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &Error::IO(ref err) => writeln!(f, "{}", err),
            &Error::Interpreter(ref err) => writeln!(f, "{}", err),
        }
    }
}