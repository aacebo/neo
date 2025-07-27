macro_rules! define_float_type {
    ($($name:ident, $type:ty)*) => {
        $(
            impl<const ROWS: usize, const COLS: usize> $crate::$name<ROWS, COLS> {
                pub fn sqrt(&self) -> Self {
                    let mut tensor = self.clone();

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            tensor.value[i][j] = tensor.value[i][j].sqrt();
                        }
                    }

                    return tensor;
                }

                pub fn cbrt(&self) -> Self {
                    let mut tensor = self.clone();

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            tensor.value[i][j] = tensor.value[i][j].cbrt();
                        }
                    }

                    return tensor;
                }

                pub fn log(&self, base: $type) -> Self {
                    let mut tensor = self.clone();

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            tensor.value[i][j] = tensor.value[i][j].log(base);
                        }
                    }

                    return tensor;
                }

                pub fn cos(&self) -> Self {
                    let mut tensor = self.clone();

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            tensor.value[i][j] = tensor.value[i][j].cos();
                        }
                    }

                    return tensor;
                }

                pub fn acos(&self) -> Self {
                    let mut tensor = self.clone();

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            tensor.value[i][j] = tensor.value[i][j].acos();
                        }
                    }

                    return tensor;
                }

                pub fn sin(&self) -> Self {
                    let mut tensor = self.clone();

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            tensor.value[i][j] = tensor.value[i][j].sin();
                        }
                    }

                    return tensor;
                }

                pub fn asin(&self) -> Self {
                    let mut tensor = self.clone();

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            tensor.value[i][j] = tensor.value[i][j].asin();
                        }
                    }

                    return tensor;
                }

                pub fn tan(&self) -> Self {
                    let mut tensor = self.clone();

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            tensor.value[i][j] = tensor.value[i][j].tan();
                        }
                    }

                    return tensor;
                }

                pub fn atan(&self) -> Self {
                    let mut tensor = self.clone();

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            tensor.value[i][j] = tensor.value[i][j].atan();
                        }
                    }

                    return tensor;
                }

                pub fn to_degrees(&self) -> Self {
                    let mut tensor = self.clone();

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            tensor.value[i][j] = tensor.value[i][j].to_degrees();
                        }
                    }

                    return tensor;
                }

                pub fn to_radians(&self) -> Self {
                    let mut tensor = self.clone();

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            tensor.value[i][j] = tensor.value[i][j].to_radians();
                        }
                    }

                    return tensor;
                }
            }
        )*
    };
}

define_float_type! {
    Float32,    f32
    Float64,    f64
}
