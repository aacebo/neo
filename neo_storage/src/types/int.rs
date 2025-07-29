use crate::{
    Serializer,
    types::{Int8, Int16, Int32, Int64, Type},
    value::Value,
};

#[derive(Debug, Clone, Copy)]
pub enum Int {
    Int8(Int8),
    Int16(Int16),
    Int32(Int32),
    Int64(Int64),
}

impl Int {
    pub fn int8() -> Self {
        return Self::Int8(Int8);
    }

    pub fn int16() -> Self {
        return Self::Int16(Int16);
    }

    pub fn int32() -> Self {
        return Self::Int32(Int32);
    }

    pub fn int64() -> Self {
        return Self::Int64(Int64);
    }

    pub fn to_type(self) -> Type {
        return Type::Int(self);
    }
}

impl Serializer<Value> for Int {
    fn serialize(&self, value: &Value) -> Result<Vec<u8>, crate::error::Error> {
        return match self {
            Int::Int8(t) => t.serialize(value),
            Int::Int16(t) => t.serialize(value),
            Int::Int32(t) => t.serialize(value),
            Int::Int64(t) => t.serialize(value),
        };
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<Value, crate::error::Error> {
        return match self {
            Self::Int8(t) => t.deserialize(bytes),
            Self::Int16(t) => t.deserialize(bytes),
            Self::Int32(t) => t.deserialize(bytes),
            Self::Int64(t) => t.deserialize(bytes),
        };
    }
}

impl Serializer<i8> for Int {
    fn serialize(&self, value: &i8) -> Result<Vec<u8>, crate::error::Error> {
        return match self {
            Self::Int8(t) => t.serialize(value),
            _ => unimplemented!(),
        };
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<i8, crate::error::Error> {
        return match self {
            Self::Int8(t) => t.deserialize(bytes),
            _ => unimplemented!(),
        };
    }
}

impl Serializer<i16> for Int {
    fn serialize(&self, value: &i16) -> Result<Vec<u8>, crate::error::Error> {
        return match self {
            Self::Int16(t) => t.serialize(value),
            _ => unimplemented!(),
        };
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<i16, crate::error::Error> {
        return match self {
            Self::Int16(t) => t.deserialize(bytes),
            _ => unimplemented!(),
        };
    }
}

impl Serializer<i32> for Int {
    fn serialize(&self, value: &i32) -> Result<Vec<u8>, crate::error::Error> {
        return match self {
            Self::Int32(t) => t.serialize(value),
            _ => unimplemented!(),
        };
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<i32, crate::error::Error> {
        return match self {
            Self::Int32(t) => t.deserialize(bytes),
            _ => unimplemented!(),
        };
    }
}

impl Serializer<i64> for Int {
    fn serialize(&self, value: &i64) -> Result<Vec<u8>, crate::error::Error> {
        return match self {
            Self::Int64(t) => t.serialize(value),
            _ => unimplemented!(),
        };
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<i64, crate::error::Error> {
        return match self {
            Self::Int64(t) => t.deserialize(bytes),
            _ => unimplemented!(),
        };
    }
}

impl std::fmt::Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Int8(t) => write!(f, "{}", t),
            Self::Int16(t) => write!(f, "{}", t),
            Self::Int32(t) => write!(f, "{}", t),
            Self::Int64(t) => write!(f, "{}", t),
        };
    }
}
