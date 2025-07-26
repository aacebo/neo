use std::ops::{Add, Div, Mul, Sub};

use num_traits::Pow;

#[derive(Debug, Clone, Copy)]
pub struct Float32<const ROWS: usize, const COLS: usize> {
    pub value: [[f32; COLS]; ROWS],
}

impl<const ROWS: usize, const COLS: usize> Float32<ROWS, COLS> {
    pub fn new() -> Self {
        return Self {
            value: [[0.0; COLS]; ROWS],
        };
    }
}

impl<const ROWS: usize, const COLS: usize> Add for Float32<ROWS, COLS> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut value = [[0.0; COLS]; ROWS];

        for i in 0..ROWS {
            for j in 0..COLS {
                value[i][j] = self.value[i][j] + rhs.value[i][j];
            }
        }

        return Self::Output { value };
    }
}

impl<const ROWS: usize, const COLS: usize> Sub for Float32<ROWS, COLS> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut value = [[0.0; COLS]; ROWS];

        for i in 0..ROWS {
            for j in 0..COLS {
                value[i][j] = self.value[i][j] - rhs.value[i][j];
            }
        }

        return Self::Output { value };
    }
}

impl<const M: usize, const N: usize, const P: usize> Mul<Float32<N, P>> for Float32<M, N> {
    type Output = Float32<M, P>;

    fn mul(self, rhs: Float32<N, P>) -> Self::Output {
        let mut value = [[0.0; P]; M];

        for i in 0..M {
            for j in 0..N {
                for n in 0..P {
                    value[i][j] += self.value[i][n] * rhs.value[n][j]
                }
            }
        }

        return Self::Output { value };
    }
}

impl<const M: usize, const N: usize, const P: usize> Div<Float32<N, P>> for Float32<M, N> {
    type Output = Float32<M, P>;

    fn div(self, rhs: Float32<N, P>) -> Self::Output {
        let mut value = [[0.0; P]; M];

        for i in 0..M {
            for j in 0..N {
                for n in 0..P {
                    value[i][j] += self.value[i][n] / rhs.value[n][j]
                }
            }
        }

        return Self::Output { value };
    }
}

impl<const ROWS: usize, const COLS: usize> Pow<f32> for Float32<ROWS, COLS> {
    type Output = Self;

    fn pow(self, rhs: f32) -> Self::Output {
        let mut value = [[0.0; COLS]; ROWS];

        for i in 0..ROWS {
            for j in 0..COLS {
                value[i][j] = self.value[i][j].pow(rhs);
            }
        }

        return Self::Output { value };
    }
}

impl<const ROWS: usize, const COLS: usize> Pow<Self> for Float32<ROWS, COLS> {
    type Output = Self;

    fn pow(self, rhs: Self) -> Self::Output {
        let mut value = [[0.0; COLS]; ROWS];

        for i in 0..ROWS {
            for j in 0..COLS {
                value[i][j] = self.value[i][j].pow(rhs.value[i][j]);
            }
        }

        return Self::Output { value };
    }
}
