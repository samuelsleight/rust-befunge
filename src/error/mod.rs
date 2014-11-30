use std::fmt::{Show, Formatter, Result};

#[deriving(Clone)]
pub enum ParserError {
    CmdError,
    FileReadError(String),
    FileEmptyError(String),
    UnexpectedChar(int, int, char),
    VarsDisabled,
    OutputError,
    OutputFileError(String)
}

impl Show for ParserError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            &ParserError::CmdError => write!(f, "Error parsing command line args"),
            &ParserError::FileReadError(ref file) => write!(f, "Unable to read file: {}", file),
            &ParserError::FileEmptyError(ref file) => write!(f, "File is empty: {}", file),

            &ParserError::UnexpectedChar(x, y, c) => {
                try!(write!(f, "Unexpected char at ({}, {}): {}\n", x, y, c));
                try!(write!(f, "This may be because of a 'j' or other reason\n"));
                write!(f, "Try passing '--exit-on-invalid' to ignore this")
            }

            &ParserError::VarsDisabled => {
                try!(write!(f, "Using 'p' or 'g' is disabled by default.\n"));
                try!(write!(f, "Pass '--enable-vars' to enable using them for variables.\n"));
                write!(f, "Disabled by default as it potentially allows invalid code.")
            }

            &ParserError::OutputError => write!(f, "Unable to write output"),
            &ParserError::OutputFileError(ref file) => write!(f, "Unable to open output file for writing: {}", file)
        }
    }
}
