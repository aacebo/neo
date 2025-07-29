#[derive(Debug, Clone)]
pub enum Value {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
}

impl TryInto<i8> for Value {
    type Error = crate::error::Error;

    fn try_into(self) -> Result<i8, Self::Error> {
        return match self {
            Self::Int8(v) => Ok(v),
            _ => Err(ValueError::from("must be Int8").into()),
        };
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        return Self::Int8(value);
    }
}

impl TryInto<i16> for Value {
    type Error = crate::error::Error;

    fn try_into(self) -> Result<i16, Self::Error> {
        return match self {
            Self::Int16(v) => Ok(v),
            _ => Err(ValueError::from("must be Int16").into()),
        };
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        return Self::Int16(value);
    }
}

impl TryInto<i32> for Value {
    type Error = crate::error::Error;

    fn try_into(self) -> Result<i32, Self::Error> {
        return match self {
            Self::Int32(v) => Ok(v),
            _ => Err(ValueError::from("must be Int32").into()),
        };
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        return Self::Int32(value);
    }
}

impl TryInto<i64> for Value {
    type Error = crate::error::Error;

    fn try_into(self) -> Result<i64, Self::Error> {
        return match self {
            Self::Int64(v) => Ok(v),
            _ => Err(ValueError::from("must be Int64").into()),
        };
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        return Self::Int64(value);
    }
}

#[derive(Debug)]
pub struct ValueError(pub String);

impl From<&str> for ValueError {
    fn from(value: &str) -> Self {
        return Self(value.to_string());
    }
}

impl Into<crate::error::Error> for ValueError {
    fn into(self) -> crate::error::Error {
        return crate::error::Error::Value(self);
    }
}

impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "[Error::Value] => {}", self.0);
    }
}

impl std::error::Error for ValueError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        return Some(self);
    }
}
