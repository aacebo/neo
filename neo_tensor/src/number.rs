macro_rules! define_number_type {
    ($($name:ident, $method:ident, $default:literal, $type:ty)*) => {
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

        impl<const ROWS: usize, const COLS: usize> $crate::Bool<ROWS, COLS> {
            $(
                pub fn $method(self) -> $name<ROWS, COLS> {
                    return self.into();
                }
            )*
        }

        impl<const ROWS: usize, const COLS: usize> num_traits::Pow<usize> for Number<ROWS, COLS> {
            type Output = Self;

            fn pow(self, rhs: usize) -> Self::Output {
                return match self {
                    $(Self::$name(lhs) => Self::$name(lhs.pow(rhs)), )*
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

                pub fn rand() -> Self {
                    let mut rng = rand::rng();
                    let mut value = [[$default; COLS]; ROWS];

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            value[i][j] = rand::Rng::random::<$type>(&mut rng);
                        }
                    }

                    return Self { value };
                }
            }

            impl<const ROWS: usize, const COLS: usize> From<$type> for $name<ROWS, COLS> {
                fn from(value: $type) -> Self {
                    return Self {
                        value: [[value; COLS]; ROWS],
                    };
                }
            }

            impl<const ROWS: usize, const COLS: usize> From<[[$type; COLS]; ROWS]> for $name<ROWS, COLS> {
                fn from(value: [[$type; COLS]; ROWS]) -> Self {
                    return Self { value };
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

            impl<const ROWS: usize, const COLS: usize> std::ops::Add<$type> for $name<ROWS, COLS> {
                type Output = Self;

                fn add(self, rhs: $type) -> Self::Output {
                    let mut value = [[$default; COLS]; ROWS];

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            value[i][j] = self.value[i][j] + rhs;
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

            impl<const ROWS: usize, const COLS: usize> std::ops::Sub<$type> for $name<ROWS, COLS> {
                type Output = Self;

                fn sub(self, rhs: $type) -> Self::Output {
                    let mut value = [[$default; COLS]; ROWS];

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            value[i][j] = self.value[i][j] - rhs;
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
                        for j in 0..P {
                            for n in 0..N {
                                value[i][j] += self.value[i][n] * rhs.value[n][j]
                            }
                        }
                    }

                    return Self::Output { value };
                }
            }

            impl<const ROWS: usize, const COLS: usize> std::ops::Mul<$type> for $name<ROWS, COLS> {
                type Output = Self;

                fn mul(self, rhs: $type) -> Self::Output {
                    let mut value = [[$default; COLS]; ROWS];

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            value[i][j] = self.value[i][j] * rhs;
                        }
                    }

                    return Self::Output { value };
                }
            }

            impl<const ROWS: usize, const COLS: usize> std::ops::Div<$type> for $name<ROWS, COLS> {
                type Output = Self;

                fn div(self, rhs: $type) -> Self::Output {
                    let mut value = [[$default; COLS]; ROWS];

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            value[i][j] = self.value[i][j] / rhs;
                        }
                    }

                    return Self::Output { value };
                }
            }

            impl<const ROWS: usize, const COLS: usize> num_traits::Pow<usize> for $name<ROWS, COLS> {
                type Output = Self;

                fn pow(self, rhs: usize) -> Self::Output {
                    let mut value = [[$default; COLS]; ROWS];

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            value[i][j] = num_traits::pow(self.value[i][j], rhs);
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
                            value[i][j] = num_traits::pow(self.value[i][j], rhs.value[i][j] as usize);
                        }
                    }

                    return Self::Output { value };
                }
            }

            impl<const ROWS: usize, const COLS: usize> std::fmt::Display for $name<ROWS, COLS> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    write!(f, "[\n")?;

                    for i in 0..ROWS {
                        write!(f, "  ")?;

                        for j in 0..COLS {
                            write!(f, "{}", self.value[i][j])?;

                            if j < COLS - 1 {
                                write!(f, ", ")?;
                            }
                        }

                        write!(f, "\n")?;
                    }

                    return write!(f, "]\n");
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
    Int8,    to_i8,     0,      i8
    Int16,   to_i16,    0,      i16
    Int32,   to_i32,    0,      i32
    Int64,   to_i64,    0,      i64
    UInt8,   to_u8,     0,      u8
    UInt16,  to_u16,    0,      u16
    UInt32,  to_u32,    0,      u32
    UInt64,  to_u64,    0,      u64
    Float32, to_f32,    0.0,    f32
    Float64, to_f64,    0.0,    f64
}
