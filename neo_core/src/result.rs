#![allow(dead_code)]

use crate::error::NeoError;

pub type Result<T> = std::result::Result<T, NeoError>;
