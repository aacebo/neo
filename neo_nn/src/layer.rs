use neo_tensor::Tensor;

pub trait Layer<
    const IN_ROWS: usize,
    const IN_COLS: usize,
    const OUT_ROWS: usize,
    const OUT_COLS: usize,
>
{
    fn forward(&mut self, input: &Tensor<IN_ROWS, IN_COLS>) -> &Tensor<OUT_ROWS, OUT_COLS>;
    fn backward(&mut self, output: &Tensor<OUT_ROWS, OUT_COLS>) -> &Tensor<IN_ROWS, IN_COLS>;
}
