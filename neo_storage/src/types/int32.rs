use crate::{SerialError, Serializer, value::Value};

#[derive(Debug, Clone, Copy)]
pub struct Int32;

impl Serializer<Value> for Int32 {
    fn serialize(&self, value: &Value) -> Result<Vec<u8>, crate::error::Error> {
        return match value {
            Value::Int32(v) => self.serialize(v),
            _ => unimplemented!(),
        };
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<Value, crate::error::Error> {
        let value = Serializer::<i32>::deserialize(self, bytes)?;
        return Ok(Value::from(value));
    }
}

impl Serializer<i32> for Int32 {
    fn serialize(&self, value: &i32) -> Result<Vec<u8>, crate::error::Error> {
        return Ok(value.to_le_bytes().to_vec());
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<i32, crate::error::Error> {
        return match TryInto::<[u8; 4]>::try_into(bytes) {
            Ok(v) => Ok(i32::from_le_bytes(v)),
            Err(err) => Err(SerialError(err.to_string()).into()),
        };
    }
}

impl std::fmt::Display for Int32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Int32");
    }
}
