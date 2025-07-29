use crate::types::Type;

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: Type,
}

impl std::fmt::Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Column[{}]: {}", self.name, self.data_type);
    }
}
