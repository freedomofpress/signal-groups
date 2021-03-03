use pyo3::prelude::*;

pub mod sho;

#[pymodule]
fn common(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<sho::Sho>()?;
    Ok(())
}
