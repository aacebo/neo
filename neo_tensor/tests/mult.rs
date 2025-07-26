use neo_tensor::{Bool, UInt8};

#[test]
pub fn should_mult_bool_uint8() {
    let a = Bool::<2, 2>::from(true);
    let b = UInt8::<2, 2>::from(3);
    let out = b * a.to_u8();

    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(out[i][j], 6);
        }
    }
}
