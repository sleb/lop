use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::num::ParseIntError;

pub type LopResult<T> = std::result::Result<T, LopError>;

#[derive(Debug, Eq, PartialEq)]
pub enum CliErrorType {
    BadDelimiter,
    MissingFile,
    MissingList,
    UnknownMode,
}

#[derive(Debug)]
pub enum LopError {
    Cli(CliErrorType),
    MalformedRangSpec,
    Parse(ParseIntError),
    File(io::Error),
}

impl Display for LopError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use CliErrorType::*;
        use LopError::*;

        let msg = match self {
            Cli(t) => match t {
                MissingFile => String::from("Missing file"),
                MissingList => String::from("Missing range list"),
                UnknownMode => String::from("Unknown lop mode"),
                BadDelimiter => String::from("Bad delimiter"),
            },
            MalformedRangSpec => String::from("Invalid range spec"),
            Parse(e) => e.to_string(),
            File(e) => e.to_string(),
        };
        write!(f, "{}", msg)
    }
}

impl From<ParseIntError> for LopError {
    fn from(e: ParseIntError) -> Self {
        LopError::Parse(e)
    }
}

impl From<io::Error> for LopError {
    fn from(e: io::Error) -> Self {
        LopError::File(e)
    }
}

impl Error for LopError {}
