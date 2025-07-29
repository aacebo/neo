pub trait Serializer<T> {
    fn serialize(&self, value: &T) -> Result<Vec<u8>, SerialError>;
    fn deserialize(&self, bytes: &[u8]) -> Result<T, SerialError>;
}

#[derive(Debug)]
pub struct SerialError(pub String);

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
