#![allow(dead_code)]

use crate::{DenseLayer, SigmoidLayer};

#[derive(Debug, Clone, Copy)]
pub struct Net<
    const IN_1: usize,
    const OUT_1: usize,
    const IN_OUT_2: usize,
    const IN_3: usize,
    const OUT_3: usize,
> {
    first: DenseLayer<IN_1, OUT_1>,
    second: SigmoidLayer<IN_OUT_2>,
    third: DenseLayer<IN_3, OUT_3>,
}
