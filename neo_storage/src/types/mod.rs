mod int;
pub use int::*;

mod int8;
pub use int8::*;

mod int16;
pub use int16::*;

mod int32;
pub use int32::*;

mod int64;
pub use int64::*;

use crate::{Serializer, value::Value};

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Int(Int),
}

impl Type {
    pub fn int8() -> Self {
        return Self::Int(Int::Int8(Int8));
    }

    pub fn int16() -> Self {
        return Self::Int(Int::Int16(Int16));
    }

    pub fn int32() -> Self {
        return Self::Int(Int::Int32(Int32));
    }

    pub fn int64() -> Self {
        return Self::Int(Int::Int64(Int64));
    }
}

impl Serializer<Value> for Type {
    fn serialize(&self, value: &Value) -> Result<Vec<u8>, crate::error::Error> {
        return match self {
            Self::Int(t) => t.serialize(value),
        };
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<Value, crate::error::Error> {
        return match self {
            Self::Int(t) => t.deserialize(bytes),
        };
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Int(t) => write!(f, "Type::{}", t),
        };
    }
}
