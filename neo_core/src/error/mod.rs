#![allow(dead_code)]

mod error_group;
pub use error_group::*;

use std::{error, fmt, io};

#[derive(Debug)]
pub enum NeoError {
    IO(io::Error),
    Fmt(fmt::Error),
    Group(ErrorGroup),
}

impl fmt::Display for NeoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Self::IO(err) => write!(f, "{}", err),
            Self::Fmt(err) => write!(f, "{}", err),
            Self::Group(err) => write!(f, "{}", err),
        };
    }
}

impl error::Error for NeoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        return match self {
            Self::IO(err) => Some(err),
            Self::Fmt(err) => Some(err),
            Self::Group(err) => Some(err),
        };
    }
}

impl From<io::Error> for NeoError {
    fn from(err: io::Error) -> Self {
        return Self::IO(err);
    }
}

impl From<fmt::Error> for NeoError {
    fn from(err: fmt::Error) -> Self {
        return Self::Fmt(err);
    }
}

impl From<ErrorGroup> for NeoError {
    fn from(err: ErrorGroup) -> Self {
        return Self::Group(err);
    }
}
