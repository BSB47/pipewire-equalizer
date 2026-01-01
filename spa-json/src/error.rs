use core::fmt;
use std::error::Error as StdError;
use std::io;

pub struct Error {
    err: Box<ErrorImpl>,
}

impl Error {
    pub(crate) fn io(error: io::Error) -> Self {
        Error {
            err: Box::new(ErrorImpl {
                code: ErrorCode::Io(error),
                line: 0,
                column: 0,
            }),
        }
    }

    pub(crate) fn syntax(code: ErrorCode, line: usize, column: usize) -> Self {
        Error {
            err: Box::new(ErrorImpl { code, line, column }),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

struct ErrorImpl {
    code: ErrorCode,
    line: usize,
    column: usize,
}

pub(crate) enum ErrorCode {
    KeyMustBeAString,
    FloatMustBeFinite,
    NumberOutOfRange,
    Io(io::Error),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match &self.err.code {
            ErrorCode::Io(err) => err.source(),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&*self.err, f)
    }
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error {
            err: Box::new(ErrorImpl {
                code: ErrorCode::Io(io::Error::other(msg.to_string())),
                line: 0,
                column: 0,
            }),
        }
    }
}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error {
            err: Box::new(ErrorImpl {
                code: ErrorCode::Io(io::Error::other(msg.to_string())),
                line: 0,
                column: 0,
            }),
        }
    }
}

impl fmt::Display for ErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.line == 0 {
            fmt::Display::fmt(&self.code, f)
        } else {
            write!(
                f,
                "{} at line {} column {}",
                self.code, self.line, self.column
            )
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCode::Io(err) => fmt::Display::fmt(err, f),
            ErrorCode::KeyMustBeAString => write!(f, "key must be a string"),
            ErrorCode::FloatMustBeFinite => write!(f, "float must be finite"),
            ErrorCode::NumberOutOfRange => write!(f, "number out of range"),
        }
    }
}
