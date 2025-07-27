use neo_tensor::{Bool, Int8, UInt8};

#[test]
pub fn should_mult_bool_uint8() {
    let a = Bool::<2, 2>::from(true);
    let b = UInt8::<2, 2>::from(3);
    let out = b * a.to_u8();
    debug_assert!(out == UInt8::from(6), "{}", out);
}

#[test]
pub fn should_mult_int8_tensor_by_int8_tensor() {
    let a = Int8::from([
        [2, 2, 2],
        [0, 1, 0]
    ]);

    let b = Int8::from([
        [2, 5],
        [6, 7],
        [1, 8]
    ]);

    let out = a * b;
    debug_assert!(out == Int8::from([[18, 40], [6, 7]]), "{}", out);
}