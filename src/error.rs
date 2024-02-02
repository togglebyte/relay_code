use std::convert::Infallible;
use std::fmt::{Debug, Display};
use std::io::Error as IoErr;
use std::str::Utf8Error;
use std::time::SystemTimeError;

use crate::try_catch::Exception;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub enum Error {
    InvalidArgs(&'static str),
    InvalidActionType,
    InvalidFieldType,
    MissingFieldLen,
    MissingFieldType,
    NoSession,
    Io(IoErr),
    Utf8(Utf8Error),
    SystemTime(SystemTimeError),
    SkillIssue(
        bool,
        &'static str,
        char,
        (u8, i32),
        String,
        [u8; 3],
        fn(u8) -> dyn Fn(u8, i32) -> u8,
    ),
    Exception(Exception),
}

impl From<Infallible> for Error {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidArgs(subcommand) => write!(f, "invalid argument\n try {subcommand}--help, but no one will hear you scream in userland"),
            Self::InvalidActionType => write!(f, "invalid action type"),
            Self::InvalidFieldType => write!(f, "invalid field type"),
            Self::MissingFieldLen => write!(f, "missing field length"),
            Self::MissingFieldType => write!(f, "missing field type"),
            Self::NoSession => write!(f, "404: your session is in another castle, or you havn't created it yet"),
            Self::Io(err) => write!(f, "{err}"),
            Self::Utf8(err) => write!(f, "{err}"),
            Self::SystemTime(err) => write!(f, "{err}"),
            Self::SkillIssue(_, _, _, _, _, _, _) => write!(f, "Lol bozo"),
            Self::Exception(err) => write!(f, "{err}"),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
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

impl From<Exception> for Error {
    fn from(value: Exception) -> Self {
        Self::Exception(value)
    }
}
