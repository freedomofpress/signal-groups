use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use serde::{Deserialize, Serialize};

use crate::api::groups;
use crate::crypto::errors::ZkGroupError;

use zkgroup;

#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct AuthCredentialPresentation {
    pub state: zkgroup::api::auth::AuthCredentialPresentation,
}

#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct AuthCredentialResponse {
    pub state: zkgroup::api::auth::AuthCredentialResponse,
}

#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct AuthCredential {
    pub state: zkgroup::api::auth::AuthCredential,
}

#[pymethods]
impl AuthCredentialPresentation {
    fn get_uuid_ciphertext(&self) -> groups::UuidCiphertext {
        groups::UuidCiphertext {
            state: self.state.get_uuid_ciphertext(),
        }
    }

    fn get_redemption_time(&self) -> zkgroup::common::simple_types::RedemptionTime {
        self.state.get_redemption_time()
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

#[pymethods]
impl AuthCredential {
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

#[pymethods]
impl AuthCredentialResponse {
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
    module.add_class::<AuthCredential>()?;
    module.add_class::<AuthCredentialResponse>()?;
    module.add_class::<AuthCredentialPresentation>()?;
    Ok(())
}
