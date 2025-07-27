use neo_tensor::{Bool, UInt8};

#[test]
pub fn should_add_bool_tensor_to_uint8_tensor() {
    let a = Bool::<2, 2>::from(true);
    let b = UInt8::<2, 2>::from(2);
    let out = b + a.to_u8();
    debug_assert!(out == UInt8::from(3), "{}", out);
}

#[test]
pub fn should_add_uint8_to_uint8_tensor() {
    let a = UInt8::<3, 3>::from(2);
    let out = a + 2;
    debug_assert!(out == UInt8::from(4), "{}", out);
}
