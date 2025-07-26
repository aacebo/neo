use crate::{Float32, Float64, Tensor, UInt8, UInt16, UInt32};

#[derive(Debug, Clone, Copy)]
pub struct Bool<const ROWS: usize, const COLS: usize> {
    pub value: [[bool; COLS]; ROWS],
}

impl<const ROWS: usize, const COLS: usize> From<bool> for Bool<ROWS, COLS> {
    fn from(value: bool) -> Self {
        return Self {
            value: [[value; COLS]; ROWS],
        };
    }
}

impl<const ROWS: usize, const COLS: usize> std::ops::Index<usize> for Bool<ROWS, COLS> {
    type Output = [bool; COLS];

    fn index(&self, index: usize) -> &Self::Output {
        return &self.value[index];
    }
}

impl<const ROWS: usize, const COLS: usize> Into<UInt8<ROWS, COLS>> for Bool<ROWS, COLS> {
    fn into(self) -> UInt8<ROWS, COLS> {
        let mut tensor = UInt8::new();

        for i in 0..ROWS {
            for j in 0..COLS {
                if self.value[i][j] {
                    tensor.value[i][j] = 1;
                }
            }
        }

        return tensor;
    }
}

impl<const ROWS: usize, const COLS: usize> Into<UInt16<ROWS, COLS>> for Bool<ROWS, COLS> {
    fn into(self) -> UInt16<ROWS, COLS> {
        let mut tensor = UInt16::new();

        for i in 0..ROWS {
            for j in 0..COLS {
                if self.value[i][j] {
                    tensor.value[i][j] = 1;
                }
            }
        }

        return tensor;
    }
}

impl<const ROWS: usize, const COLS: usize> Into<UInt32<ROWS, COLS>> for Bool<ROWS, COLS> {
    fn into(self) -> UInt32<ROWS, COLS> {
        let mut tensor = UInt32::new();

        for i in 0..ROWS {
            for j in 0..COLS {
                if self.value[i][j] {
                    tensor.value[i][j] = 1;
                }
            }
        }

        return tensor;
    }
}

impl<const ROWS: usize, const COLS: usize> Into<Float32<ROWS, COLS>> for Bool<ROWS, COLS> {
    fn into(self) -> Float32<ROWS, COLS> {
        let mut tensor = Float32::new();

        for i in 0..ROWS {
            for j in 0..COLS {
                if self.value[i][j] {
                    tensor.value[i][j] = 1.0;
                }
            }
        }

        return tensor;
    }
}

impl<const ROWS: usize, const COLS: usize> Into<Float64<ROWS, COLS>> for Bool<ROWS, COLS> {
    fn into(self) -> Float64<ROWS, COLS> {
        let mut tensor = Float64::new();

        for i in 0..ROWS {
            for j in 0..COLS {
                if self.value[i][j] {
                    tensor.value[i][j] = 1.0;
                }
            }
        }

        return tensor;
    }
}

impl<const ROWS: usize, const COLS: usize> std::fmt::Display for Bool<ROWS, COLS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        return write!(f, "{:#?}", self.value);
    }
}

impl<const ROWS: usize, const COLS: usize> PartialEq for Bool<ROWS, COLS> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..ROWS {
            for j in 0..COLS {
                if self.value[i][j] != other.value[i][j] {
                    return false;
                }
            }
        }

        return true;
    }
}

impl<const ROWS: usize, const COLS: usize> PartialEq<Tensor<ROWS, COLS>> for Bool<ROWS, COLS> {
    fn eq(&self, other: &Tensor<ROWS, COLS>) -> bool {
        return match other {
            Tensor::Bool(t) => t == self,
            _ => false,
        };
    }
}
