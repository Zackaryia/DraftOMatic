use pyo3::prelude::*;
mod main;
use pyo3::types::PyList;
use serde::ser::{Serialize, SerializeStruct, Serializer};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn evaluate(draft: Vec<u16>, depth: u16) -> String {
    serde_json::to_string(&main::eval_state(draft, depth)).unwrap()

}

/// A Python module implemented in Rust.
#[pymodule]
fn draftomatic(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(evaluate, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

    Ok(())
}
