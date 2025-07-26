mod float32;
use std::ops::{Add, Div, Mul, Sub};

pub use float32::*;
use num_traits::Pow;

#[derive(Debug, Clone, Copy)]
pub enum Tensor<const ROWS: usize, const COLS: usize> {
    Float32(Float32<ROWS, COLS>),
}

impl<const ROWS: usize, const COLS: usize> Add<Float32<ROWS, COLS>> for Tensor<ROWS, COLS> {
    type Output = Float32<ROWS, COLS>;

    fn add(self, rhs: Float32<ROWS, COLS>) -> Self::Output {
        return match self {
            Self::Float32(lhs) => lhs + rhs,
        };
    }
}

impl<const ROWS: usize, const COLS: usize> Sub<Float32<ROWS, COLS>> for Tensor<ROWS, COLS> {
    type Output = Float32<ROWS, COLS>;

    fn sub(self, rhs: Float32<ROWS, COLS>) -> Self::Output {
        return match self {
            Self::Float32(lhs) => lhs - rhs,
        };
    }
}

impl<const M: usize, const N: usize, const P: usize> Mul<Float32<N, P>> for Tensor<M, N> {
    type Output = Float32<M, P>;

    fn mul(self, rhs: Float32<N, P>) -> Self::Output {
        return match self {
            Self::Float32(lhs) => lhs * rhs,
        };
    }
}

impl<const M: usize, const N: usize, const P: usize> Div<Float32<N, P>> for Tensor<M, N> {
    type Output = Float32<M, P>;

    fn div(self, rhs: Float32<N, P>) -> Self::Output {
        return match self {
            Self::Float32(lhs) => lhs / rhs,
        };
    }
}

impl<const ROWS: usize, const COLS: usize> Pow<f32> for Tensor<ROWS, COLS> {
    type Output = Float32<ROWS, COLS>;

    fn pow(self, rhs: f32) -> Self::Output {
        return match self {
            Self::Float32(lhs) => lhs.pow(rhs),
        };
    }
}

impl<const ROWS: usize, const COLS: usize> Pow<Float32<ROWS, COLS>> for Tensor<ROWS, COLS> {
    type Output = Float32<ROWS, COLS>;

    fn pow(self, rhs: Float32<ROWS, COLS>) -> Self::Output {
        return match self {
            Self::Float32(lhs) => lhs.pow(rhs),
        };
    }
}
