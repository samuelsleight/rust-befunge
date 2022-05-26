use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    EOF,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::EOF => write!(
                f,
                "Interpreter ran out of input without reaching an end state"
            ),
        }
    }
}
