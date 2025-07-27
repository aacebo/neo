use neo_tensor::Int64;

#[test]
pub fn should_convert_i64_tensor_to_abs() {
    let a = Int64::from([[0, -1, 200], [-500, 3000, 0]]);
    let out = a.abs();
    debug_assert_eq!(out, Int64::from([[0, 1, 200], [500, 3000, 0]]));
}
