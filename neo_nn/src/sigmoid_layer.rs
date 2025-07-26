use crate::DenseLayer;

#[derive(Debug, Clone, Copy)]
pub struct SigmoidLayer<const N: usize> {
    pub(crate) grad: [f32; N],
    pub(crate) last_input: [f32; N],
    pub(crate) last_output: [f32; N],
}

impl<const N: usize> SigmoidLayer<N> {
    pub fn new() -> Self {
        return Self {
            grad: [0.0; N],
            last_input: [0.0; N],
            last_output: [0.0; N],
        };
    }

    pub fn forward(&mut self, input: &[f32; N]) -> [f32; N] {
        self.last_input = input.clone();
        let mut output = [0.0; N];

        for i in 0..N {
            output[i] = 1.0 / (1.0 + (-input[i].exp()))
        }

        self.last_output = output;
        return output;
    }

    pub fn backward(&mut self, chain_grad: &[f32; N]) {
        self.grad = [0.0; N];

        for i in 0..N {
            self.grad[i] = self.last_output[i] * (1.0 - self.last_output[i]) * chain_grad[i];
        }
    }

    pub fn backward_from<const OUT: usize>(&mut self, prev_layer: &DenseLayer<N, OUT>) {
        self.grad = [0.0; N];

        for i in 0..N {
            for j in 0..OUT {
                let grad = self.last_output[j] * (1.0 - self.last_output[j]);
                let chain_grad = prev_layer.neurons[j].weights[i] * prev_layer.neurons[j].wgrad[i]
                    / prev_layer.last_input[i];
                self.grad[i] += grad * chain_grad;
            }
        }
    }
}
