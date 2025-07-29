use crate::SerialError;

#[derive(Debug)]
pub enum Error {
    Serial(SerialError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Serial(err) => write!(f, "{}", err),
        };
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        return Some(self);
    }
}
