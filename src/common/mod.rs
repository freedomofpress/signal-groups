use pyo3::prelude::*;

//pub mod errors;
pub mod sho;

#[pymodule]
fn common(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<sho::Sho>()?;
    // module.add(
    //     "ZkGroupException",
    //     py.get_type::<errors::ZkGroupException>(),
    // )?;
    Ok(())
}
