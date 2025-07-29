use std::io;

use crate::{Column, value::Value};

#[derive(Debug, Clone)]
pub struct Table<const N: usize> {
    pub name: String,
    pub columns: [Column; N],
}

impl<const N: usize> Table<N> {
    pub fn find_by_id(&self, _id: &str) -> Result<Option<[Value; N]>, io::Error> {
        unimplemented!()
    }

    pub fn insert(&self, _row: [Value; N]) -> Result<(), io::Error> {
        unimplemented!()
    }

    pub fn update(&self, _id: &str, _row: [Value; N]) -> Result<(), io::Error> {
        unimplemented!()
    }

    pub fn delete(&self, _id: &str) -> Result<(), io::Error> {
        unimplemented!()
    }
}
