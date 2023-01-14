# n use the torch crate in your Rust code to interact with PyTorch. Here's an example of how to define and manipulate a simple tensor:

use torch::prelude::*;

fn main() {
    // Create a tensor with shape (2, 3) filled with ones
    let tensor = torch::ones([2, 3], TensorOptions::dtype(torch::kFloat32));

    // Print the tensor
    println!("{}", tensor);

    // Perform an operation on the tensor
    let tensor = tensor * 2.0;
    println!("{}", tensor);
}
