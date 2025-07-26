mod number;
pub use number::*;

mod bool;
pub use bool::*;

#[derive(Debug, Clone, Copy)]
pub enum Tensor<const ROWS: usize, const COLS: usize> {
    Bool(Bool<ROWS, COLS>),
    Number(Number<ROWS, COLS>),
}

impl<const ROWS: usize, const COLS: usize> std::fmt::Display for Tensor<ROWS, COLS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        return match self {
            Self::Bool(t) => write!(f, "{}", t),
            Self::Number(t) => write!(f, "{}", t),
        };
    }
}

impl<const ROWS: usize, const COLS: usize> PartialEq for Tensor<ROWS, COLS> {
    fn eq(&self, other: &Self) -> bool {
        return match self {
            Self::Bool(t) => t == other,
            Self::Number(t) => t == other,
        };
    }
}
