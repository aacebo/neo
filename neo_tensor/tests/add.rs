use neo_tensor::{Bool, UInt8};

#[test]
pub fn should_add_bool_tensor_to_uint8_tensor() {
    let a = Bool::<2, 2>::from(true);
    let b = UInt8::<2, 2>::from(2);
    let out = b + a.to_uint8();

    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(out[i][j], 3);
        }
    }
}

#[test]
pub fn should_add_uint8_to_uint8_tensor() {
    let a = UInt8::<2, 2>::from(2);
    let out = a + 2;
    assert!(out == UInt8::<2, 2>::from(4));
}
