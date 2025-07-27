use neo_tensor::{Bool, UInt8};

#[test]
pub fn should_mult_bool_uint8() {
    let a = Bool::<2, 2>::from(true);
    let b = UInt8::<2, 2>::from(3);
    let out = b * a.to_u8();
    assert!(out == UInt8::from(6));
}
