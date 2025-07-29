use crate::{SerialError, Serializer, types::Type};

#[derive(Debug, Clone)]
pub enum Value {
    Int8(i8),
}

impl Value {
    pub fn serialize(&self, data_type: Type) -> Result<Vec<u8>, SerialError> {
        return match self {
            Self::Int8(v) => data_type.serialize(v),
        };
    }

    pub fn deserialize(data_type: Type, bytes: &[u8]) -> Result<Self, SerialError> {
        return match data_type {
            Type::Int8(t) => {
                let v = t.deserialize(bytes)?;
                Ok(Self::Int8(v))
            }
        };
    }
}
