use neo_tensor::{Bool, UInt8};

#[test]
pub fn should_add_bool_uint8() {
    let a = Bool::<2, 2>::from(true);
    let b = UInt8::<2, 2>::from(2);
    let out = b + a.into();

    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(out[i][j], 3);
        }
    }
}
