# You can also use torch::autograd to perform gradient computation and training of deep learning models.


use torch::prelude::*;
use torch::nn::{Linear,Module};

struct Net {
    fc: Linear,
}

impl Net {
    fn new(in_features: i64, out_features: i64) -> Net {
        let fc = Linear::new(in_features, out_features);
        Net { fc }
    }
}

impl Module for Net {
    fn forward(&self, x: &Tensor) -> Tensor {
        self.fc.forward(x)
    }
}

#// Note that this is just a simple example and there are many more features and functionality provided by the torch crate. You can find more information and examples in the official documentation: https://docs.rs/torch/
