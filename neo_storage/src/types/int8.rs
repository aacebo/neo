use crate::{SerialError, Serializer};

#[derive(Debug, Clone, Copy)]
pub struct Int8;

impl Serializer<i8> for Int8 {
    fn serialize(&self, value: &i8) -> Result<Vec<u8>, SerialError> {
        return Ok(value.to_le_bytes().to_vec());
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<i8, SerialError> {
        return match TryInto::<[u8; 1]>::try_into(bytes) {
            Ok(v) => Ok(i8::from_le_bytes(v)),
            Err(err) => Err(SerialError(err.to_string())),
        };
    }
}

impl std::fmt::Display for Int8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Int8");
    }
}
