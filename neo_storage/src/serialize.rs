pub trait Serializer<T> {
    fn serialize(&self, value: &T) -> Result<Vec<u8>, crate::error::Error>;
    fn deserialize(&self, bytes: &[u8]) -> Result<T, crate::error::Error>;
}

#[derive(Debug)]
pub struct SerialError(pub String);

impl From<&str> for SerialError {
    fn from(value: &str) -> Self {
        return Self(value.to_string());
    }
}

impl Into<crate::error::Error> for SerialError {
    fn into(self) -> crate::error::Error {
        return crate::error::Error::Serial(self);
    }
}

impl std::fmt::Display for SerialError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "[Error::Serial] => {}", self.0);
    }
}

impl std::error::Error for SerialError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        return Some(self);
    }
}
