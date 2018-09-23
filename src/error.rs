use std::{
    io,
    fmt::{self, Formatter, Display}
};

#[derive(Debug)]
pub enum Error {
    IO(io::Error)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &Error::IO(ref err) => writeln!(f, "{}", err)
        }
    }
}