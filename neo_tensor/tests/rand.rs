use neo_tensor::Int32;

#[test]
pub fn should_generate_random_i32_tensor() {
    let a = Int32::<5, 1>::rand();
    let b = Int32::<5, 1>::rand();
    assert_ne!(a, b);
}
