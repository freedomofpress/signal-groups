use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use serde::{Deserialize, Serialize};

use zkgroup;

use crate::crypto::errors::ZkGroupError;

//TODO: Default, PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
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

    fn serialize(&self, py: Python) -> Result<PyObject, ZkGroupError> {
        let bytes = bincode::serialize(&self).expect("could not serialize to bytes");
        Ok(PyBytes::new(py, &bytes).into())
    }

    #[staticmethod]
    fn deserialize(bytes: &[u8]) -> PyResult<Self> {
        match bincode::deserialize(bytes) {
            Ok(result) => Ok(result),
            Err(_) => Err(PyValueError::new_err("cannot deserialize")),
        }
    }
}

pub fn init_submodule(module: &PyModule) -> PyResult<()> {
    module.add_class::<ProfileKeyStruct>()?;
    Ok(())
}
