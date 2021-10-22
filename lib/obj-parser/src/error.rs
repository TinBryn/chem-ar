use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    line: String,
}

impl Error {
    pub fn new(kind: ErrorKind, line: String) -> Self {
        Self { kind, line }
    }

    pub fn invalid(line: String) -> Self {
        Self {
            kind: ErrorKind::InvalidObjData,
            line,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} on line: {}", self.kind, self.line)
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
    InvalidObjData,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::ParseInt(error) => std::fmt::Display::fmt(error, f),
            ErrorKind::ParseFloat(error) => std::fmt::Display::fmt(error, f),
            ErrorKind::InvalidObjData => write!(f, "Invalid .obj data"),
        }
    }
}

impl std::error::Error for ErrorKind {}

impl From<ParseIntError> for ErrorKind {
    fn from(error: ParseIntError) -> Self {
        ErrorKind::ParseInt(error)
    }
}

impl From<ParseFloatError> for ErrorKind {
    fn from(error: ParseFloatError) -> Self {
        ErrorKind::ParseFloat(error)
    }
}
