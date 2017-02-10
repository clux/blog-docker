use std::fmt;
use std::io;
use std::str::Utf8Error;
use serde_json;
use glob;

/// Main error types not related to the HTTP server
#[derive(Debug)]
pub enum BlogError {
    Io(io::Error),
    Parse(serde_json::error::Error),
    Pattern(glob::PatternError),
    Glob(glob::GlobError),
    Unicode(Utf8Error),
}

/// Type alias to stop having to type out BlogError everywhere
pub type BlogResult<T> = Result<T, BlogError>;

// Format implementation used when printing an error
impl fmt::Display for BlogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BlogError::Io(ref err) => err.fmt(f),
            BlogError::Parse(ref err) => err.fmt(f),
            BlogError::Pattern(ref err) => err.fmt(f),
            BlogError::Glob(ref err) => err.fmt(f),
            BlogError::Unicode(ref err) => err.fmt(f),
        }
    }
}

// Absorb error types
impl From<io::Error> for BlogError {
    fn from(err: io::Error) -> BlogError {
        BlogError::Io(err)
    }
}
impl From<serde_json::error::Error> for BlogError {
    fn from(err: serde_json::error::Error) -> BlogError {
        BlogError::Parse(err)
    }
}

impl From<glob::PatternError> for BlogError {
    fn from(err: glob::PatternError) -> BlogError {
        BlogError::Pattern(err)
    }
}
impl From<glob::GlobError> for BlogError {
    fn from(err: glob::GlobError) -> BlogError {
        BlogError::Glob(err)
    }
}
impl From<Utf8Error> for BlogError {
    fn from(err: Utf8Error) -> BlogError {
        BlogError::Unicode(err)
    }
}
