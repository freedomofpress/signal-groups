use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use serde::{Deserialize, Serialize};

use zkgroup;

use crate::crypto::errors::ZkGroupError;

//TODO: PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct UidStruct {
    pub state: zkgroup::crypto::uid_struct::UidStruct,
}

#[pymethods]
impl UidStruct {
    #[new]
    fn new(uid_bytes: zkgroup::common::simple_types::UidBytes) -> UidStruct {
        UidStruct {
            state: zkgroup::crypto::uid_struct::UidStruct::new(uid_bytes),
        }
    }

    // TODO: do we need from_M2 in public API?

    pub fn to_bytes(&self, py: Python) -> PyObject {
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
    module.add_class::<UidStruct>()?;
    Ok(())
}
