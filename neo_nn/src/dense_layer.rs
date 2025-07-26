use crate::Neuron;

#[derive(Debug, Clone, Copy)]
pub struct DenseLayer<const IN: usize, const OUT: usize> {
    pub(crate) last_input: [f32; IN],
    pub(crate) neurons: [Neuron<IN>; OUT],
}

impl<const IN: usize, const OUT: usize> DenseLayer<IN, OUT> {
    pub fn new() -> Self {
        return Self {
            last_input: [0.0; IN],
            neurons: [Neuron::new(); OUT],
        };
    }

    pub fn zero_grad(&mut self) {
        for neuron in self.neurons.iter_mut() {
            neuron.zero_grad();
        }
    }

    pub fn forward(&mut self, input: &[f32; IN]) -> [f32; OUT] {
        self.last_input = input.clone();
        let mut output = [0.0; OUT];

        for i in 0..OUT {
            output[i] = self.neurons[i].forward(input);
        }

        return output;
    }

    pub fn backward(&mut self, grad: &[f32; OUT]) {
        for i in 0..OUT {
            self.neurons[i].backward(&grad[i], &self.last_input);
        }
    }

    pub fn decend(&mut self, learning_rate: &f32) {
        for i in 0..OUT {
            self.neurons[i].decend(learning_rate);
        }
    }
}
