macro_rules! define_signed_type {
    ($($name:ident, $type:ty)*) => {
        $(
            impl<const ROWS: usize, const COLS: usize> $crate::$name<ROWS, COLS> {
                pub fn abs(&self) -> Self {
                    let mut tensor = self.clone();

                    for i in 0..ROWS {
                        for j in 0..COLS {
                            tensor.value[i][j] = tensor.value[i][j].abs();
                        }
                    }

                    return tensor;
                }
            }
        )*
    };
}

define_signed_type! {
    Int8,       i8
    Int16,      i16
    Int32,      i32
    Int64,      i64
    Float32,    f32
    Float64,    f64
}
