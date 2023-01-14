# You can also use the pyo3 library to import the PyTorch library and use its functions, such as creating tensors, performing computations, and training models.
Here's an example of creating a tensor in Rust:

let gil = Python::acquire_gil();
let py = gil.python();
let torch = py.import("torch")?;
let tensor = torch.call("tensor", (vec![1, 2, 3],))?;

# This code imports the PyTorch library and creates a tensor with the values [1, 2, 3].
Please note that, this is a basic example and you can use pyo3 to interact with other python libraries as well.
