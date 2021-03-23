use pyo3::prelude::*;
use pyo3::types::PyBytes;

use zkgroup;

#[pyclass]
pub struct Sho {
    pub state: zkgroup::common::sho::Sho,
}

#[pymethods]
impl Sho {
    #[new]
    fn new(label: &[u8], data: &[u8]) -> Self {
        Self {
            state: zkgroup::common::sho::Sho::new(label, data),
        }
    }

    fn squeeze(&mut self, outlen: usize, py: Python) -> PyObject {
        let output = self.state.squeeze(outlen);
        PyBytes::new(py, &output).into()
    }

    // TODO: Does get_point, get_point_single_elligator, get_scalar need to be part of the public API?
}

pub fn init_submodule(module: &PyModule) -> PyResult<()> {
    module.add_class::<Sho>()?;
    Ok(())
}
