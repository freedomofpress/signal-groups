use pyo3::prelude::*;
use pyo3::types::PyBytes;

use zkgroup;

//TODO: Default, PartialEq, Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
pub struct ProfileKeyStruct {
    pub state: zkgroup::crypto::profile_key_struct::ProfileKeyStruct,
}

#[pymethods]
impl ProfileKeyStruct {
    #[new]
    fn new(
        profile_key_bytes: zkgroup::common::simple_types::ProfileKeyBytes,
        uid_bytes: zkgroup::common::simple_types::UidBytes,
    ) -> ProfileKeyStruct {
        ProfileKeyStruct {
            state: zkgroup::crypto::profile_key_struct::ProfileKeyStruct::new(
                profile_key_bytes,
                uid_bytes,
            ),
        }
    }

    // TODO: do we need calc_M3 in public API? (returns RistrettoPoint)

    fn to_bytes(&self, py: Python) -> PyObject {
        PyBytes::new(py, &self.state.to_bytes()).into()
    }
}

pub fn init_submodule(module: &PyModule) -> PyResult<()> {
    module.add_class::<ProfileKeyStruct>()?;
    Ok(())
}
