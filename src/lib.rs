use pyo3::prelude::*;

mod api;
mod common;
mod crypto;

/// Signal groups in Python
///
/// This Rust extension provides Python bindings for the Rust crate
/// zkgroups.
#[pymodule]
fn signal_groups(_py: Python, _module: &PyModule) -> PyResult<()> {
    Ok(())
}
