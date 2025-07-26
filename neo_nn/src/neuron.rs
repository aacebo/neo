use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Neuron<const N: usize> {
    pub(crate) bias: f32,
    pub(crate) bgrad: f32,
    pub(crate) weights: [f32; N],
    pub(crate) wgrad: [f32; N],
}

impl<const N: usize> Neuron<N> {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut weights = [0.0; N];

        for i in 0..N {
            weights[i] = rng.random_range(-1.0..1.0);
        }

        return Self {
            bias: 0.01 * rng.random_range(-1.0..1.0),
            bgrad: 0.0,
            weights,
            wgrad: [0.0; N],
        };
    }

    pub fn zero_grad(&mut self) {
        self.wgrad = [0.0; N];
        self.bgrad = 0.0;
    }

    pub fn forward(&self, input: &[f32; N]) -> f32 {
        let mut total = self.bias;

        for i in 0..N {
            total += input[i] * self.weights[i];
        }

        return total;
    }

    pub fn backward(&mut self, grad: &f32, last_input: &[f32; N]) {
        self.bgrad += grad;

        for i in 0..N {
            self.wgrad[i] += grad * last_input[i];
        }
    }

    pub fn decend(&mut self, learning_rate: &f32) {
        self.bias -= self.bgrad * learning_rate;

        for i in 0..N {
            self.weights[i] -= self.wgrad[i] * learning_rate;
        }
    }
}
