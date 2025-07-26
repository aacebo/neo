macro_rules! define_number_type {
    ($($name:ident $default:literal $type:ty)*) => {
        #[derive(Debug, Clone, Copy)]
        pub enum Number<const ROWS: usize, const COLS: usize> {
            $($name($name<ROWS, COLS>), )*
        }

        impl<const ROWS: usize, const COLS: usize> std::fmt::Display for Number<ROWS, COLS> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                return match self {
                    $(Self::$name(t) => write!(f, "{}", t), )*
                };
            }
        }

        impl<const ROWS: usize, const COLS: usize> PartialEq for Number<ROWS, COLS> {
            fn eq(&self, other: &Self) -> bool {
                return match self {
                    $(Self::$name(t) => t == other, )*
                };
            }
        }

        impl<const ROWS: usize, const COLS: usize> PartialEq<$crate::Tensor<ROWS, COLS>> for Number<ROWS, COLS> {
            fn eq(&self, other: &$crate::Tensor<ROWS, COLS>) -> bool {
                return match other {
                    $crate::Tensor::Number(t) => t == self,
                    _ => false,
                };
            }
        }

        $(
            impl<const ROWS: usize, const COLS: usize> std::ops::Add<$name<ROWS, COLS>> for Number<ROWS, COLS> {
                type Output = $name<ROWS, COLS>;

                fn add(self, rhs: $name<ROWS, COLS>) -> Self::Output {
                    return match self {
                        Self::$name(lhs) => lhs + rhs,
                        _ => panic!("{} => invalid operation", stringify!($name)),
                    };
                }
            }

            impl<const ROWS: usize, const COLS: usize> std::ops::Sub<$name<ROWS, COLS>> for Number<ROWS, COLS> {
                type Output = $name<ROWS, COLS>;

                fn sub(self, rhs: $name<ROWS, COLS>) -> Self::Output {
                    return match self {
                        Self::$name(lhs) => lhs - rhs,
                        _ => panic!("{} => invalid operation", stringify!($name)),
                    };
                }
            }

            impl<const M: usize, const N: usize, const P: usize> std::ops::Mul<$name<N, P>> for Number<M, N> {
                type Output = $name<M, P>;

                fn mul(self, rhs: $name<N, P>) -> Self::Output {
                    return match self {
                        Self::$name(lhs) => lhs * rhs,
                        _ => panic!("{} => invalid operation", stringify!($name)),
                    };
                }
            }

            impl<const M: usize, const N: usize, const P: usize> std::ops::Div<$name<N, P>> for Number<M, N> {
                type Output = $name<M, P>;

                fn div(self, rhs: $name<N, P>) -> Self::Output {
                    return match self {
                        Self::$name(lhs) => lhs / rhs,
                        _ => panic!("{} => invalid operation", stringify!($name)),
                    };
                }
            }

            impl<const ROWS: usize, const COLS: usize> num_traits::Pow<$type> for Number<ROWS, COLS> {
                type Output = $name<ROWS, COLS>;

                fn pow(self, rhs: $type) -> Self::Output {
                    return match self {
                        Self::$name(lhs) => lhs.pow(rhs),
                        _ => panic!("{} => invalid operation", stringify!($name)),
                    };
                }
            }

            impl<const ROWS: usize, const COLS: usize> num_traits::Pow<$name<ROWS, COLS>> for Number<ROWS, COLS> {
                type Output = $name<ROWS, COLS>;

                fn pow(self, rhs: $name<ROWS, COLS>) -> Self::Output {
                    return match self {
                        Self::$name(lhs) => lhs.pow(rhs),
                        _ => panic!("{} => invalid operation", stringify!($name)),
                    };
                }
            }
        )*

        $(
            #[derive(Debug, Clone, Copy)]
            pub struct $name<const ROWS: usize, const COLS: usize> {
                pub value: [[$type; COLS]; ROWS],
            }

            impl<const ROWS: usize, const COLS: usize> $name<ROWS, COLS> {
                pub fn new() -> Self {
                    return Self {
                        value: [[$default; COLS]; ROWS],
                    };
                }
            }

            impl<const ROWS: usize, const COLS: usize> From<$type> for $name<ROWS, COLS> {
                fn from(value: $type) -> Self {
                    return Self {
                        value: [[value; COLS]; ROWS],
                    };
                }
            }

            impl<const ROWS: usize, const COLS: usize> std::ops::Index<usize> for $name<ROWS, COLS> {
                type Output = [$type; COLS];

                fn index(&self, index: usize) -> &Self::Output {
                    return &self.value[index];
                }
            }

            impl<const ROWS: usize, const COLS: usize> Into<$crate::Bool<ROWS, COLS>> for $name<ROWS, COLS> {
                fn into(self) -> $crate::Bool<ROWS, COLS> {
                    let mut value = [[false; COLS]; ROWS];

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            if self.value[i][j] > $default {
                                value[i][j] = true;
                            }
                        }
                    }

                    return $crate::Bool { value };
                }
            }

            impl<const ROWS: usize, const COLS: usize> std::ops::Add for $name<ROWS, COLS> {
                type Output = Self;

                fn add(self, rhs: Self) -> Self::Output {
                    let mut value = [[$default; COLS]; ROWS];

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            value[i][j] = self.value[i][j] + rhs.value[i][j];
                        }
                    }

                    return Self::Output { value };
                }
            }

            impl<const ROWS: usize, const COLS: usize> std::ops::Sub for $name<ROWS, COLS> {
                type Output = Self;

                fn sub(self, rhs: Self) -> Self::Output {
                    let mut value = [[$default; COLS]; ROWS];

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            value[i][j] = self.value[i][j] - rhs.value[i][j];
                        }
                    }

                    return Self::Output { value };
                }
            }

            impl<const M: usize, const N: usize, const P: usize> std::ops::Mul<$name<N, P>> for $name<M, N> {
                type Output = $name<M, P>;

                fn mul(self, rhs: $name<N, P>) -> Self::Output {
                    let mut value = [[$default; P]; M];

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

            impl<const M: usize, const N: usize, const P: usize> std::ops::Div<$name<N, P>> for $name<M, N> {
                type Output = $name<M, P>;

                fn div(self, rhs: $name<N, P>) -> Self::Output {
                    let mut value = [[$default; P]; M];

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

            impl<const ROWS: usize, const COLS: usize> num_traits::Pow<$type> for $name<ROWS, COLS> {
                type Output = Self;

                fn pow(self, rhs: $type) -> Self::Output {
                    let mut value = [[$default; COLS]; ROWS];

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            value[i][j] = num_traits::Pow::pow(self.value[i][j], rhs);
                        }
                    }

                    return Self::Output { value };
                }
            }

            impl<const ROWS: usize, const COLS: usize> num_traits::Pow<Self> for $name<ROWS, COLS> {
                type Output = Self;

                fn pow(self, rhs: Self) -> Self::Output {
                    let mut value = [[$default; COLS]; ROWS];

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            value[i][j] = num_traits::Pow::pow(self.value[i][j], rhs.value[i][j]);
                        }
                    }

                    return Self::Output { value };
                }
            }

            impl<const ROWS: usize, const COLS: usize> std::fmt::Display for $name<ROWS, COLS> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    return write!(f, "{:#?}", self.value);
                }
            }

            impl<const ROWS: usize, const COLS: usize> PartialEq for $name<ROWS, COLS> {
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

            impl<const ROWS: usize, const COLS: usize> PartialEq<Number<ROWS, COLS>> for $name<ROWS, COLS> {
                fn eq(&self, other: &Number<ROWS, COLS>) -> bool {
                    return match other {
                        Number::$name(t) => t == self,
                        _ => false,
                    };
                }
            }
        )*
    };
}

define_number_type! {
    UInt8    0   u8
    UInt16   0   u16
    UInt32   0   u32
    Float32 0.0  f32
    Float64 0.0  f64
}
