# defines a rust function add_numbers which takes two i32 arguments and returns their sum as an i32. The function is then wrapped with the wrap_pyfunction! macro from the pyo3 library and added to a PyModule.


extern crate pyo3;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[pymodule]
fn rust_pytorch(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(add_numbers))?;

    Ok(())
}
