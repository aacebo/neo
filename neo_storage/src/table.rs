use crate::{Column, value::Value};

#[derive(Debug, Clone)]
pub struct Table<const N: usize> {
    pub columns: [Column; N],
    pub rows: Vec<[Value; N]>,
}

impl<const N: usize> From<[Column; N]> for Table<N> {
    fn from(columns: [Column; N]) -> Self {
        return Self {
            columns,
            rows: vec![],
        };
    }
}
