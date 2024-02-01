use std::fmt::Display;
use std::io::Error as IoErr;
use std::str::Utf8Error;
use std::time::SystemTimeError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidArgs,
    InvalidActionType,
    InvalidFieldType,
    MissingFieldLen,
    MissingFieldType,
    NoEntity,
    Io(IoErr),
    Utf8(Utf8Error),
    SystemTime(SystemTimeError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidArgs => write!(f, "invalid argument"),
            Self::InvalidActionType => write!(f, "invalid action type"),
            Self::InvalidFieldType => write!(f, "invalid field type"),
            Self::MissingFieldLen => write!(f, "missing field length"),
            Self::MissingFieldType => write!(f, "missing field type"),
            Self::NoEntity => write!(f, "no entity"),
            Self::Io(err) => write!(f, "{err}"),
            Self::Utf8(err) => write!(f, "{err}"),
            Self::SystemTime(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<IoErr> for Error {
    fn from(err: IoErr) -> Self {
        Self::Io(err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Self::Utf8(err)
    }
}

impl From<SystemTimeError> for Error {
    fn from(err: SystemTimeError) -> Self {
        Self::SystemTime(err)
    }
}
