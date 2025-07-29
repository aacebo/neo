mod int8;
pub use int8::*;

use crate::Serializer;

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Int8(Int8),
}

impl Type {
    pub fn int8() -> Self {
        return Self::Int8(Int8);
    }
}

impl Serializer<i8> for Type {
    fn serialize(&self, value: &i8) -> Result<Vec<u8>, crate::SerialError> {
        return match self {
            Self::Int8(t) => t.serialize(value),
        };
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<i8, crate::SerialError> {
        return match self {
            Self::Int8(t) => t.deserialize(bytes),
        };
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Int8(t) => write!(f, "Type::{}", t),
        };
    }
}
