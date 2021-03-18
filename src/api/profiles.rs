use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use serde::{Deserialize, Serialize};

// use crate::crypto::profile_key_struct::ProfileKeyStruct;

use crate::api::groups;
use crate::crypto::errors::ZkGroupError;

use zkgroup;

//TODO: Default, PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct ProfileKey {
    pub state: zkgroup::api::profiles::ProfileKey,
}

#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct ProfileKeyCommitment {
    pub state: zkgroup::api::profiles::ProfileKeyCommitment,
}

#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct ProfileKeyCredentialPresentation {
    pub state: zkgroup::api::profiles::ProfileKeyCredentialPresentation,
}

#[pymethods]
impl ProfileKeyCredentialPresentation {
    fn get_uuid_ciphertext(&self) -> groups::UuidCiphertext {
        groups::UuidCiphertext {
            state: self.state.get_uuid_ciphertext(),
        }
    }

    fn get_profile_key_ciphertext(&self) -> groups::ProfileKeyCiphertext {
        groups::ProfileKeyCiphertext {
            state: self.state.get_profile_key_ciphertext(),
        }
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

#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct ProfileKeyCredentialRequest {
    pub state: zkgroup::api::profiles::ProfileKeyCredentialRequest,
}

#[pymethods]
impl ProfileKeyCredentialRequest {
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

#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct ProfileKeyCredentialRequestContext {
    pub state: zkgroup::api::profiles::ProfileKeyCredentialRequestContext,
}

#[pymethods]
impl ProfileKeyCredentialRequestContext {
    fn get_request(&self) -> ProfileKeyCredentialRequest {
        ProfileKeyCredentialRequest {
            state: self.state.get_request(),
        }
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

#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct ProfileKeyCredentialResponse {
    pub state: zkgroup::api::profiles::ProfileKeyCredentialResponse,
}

#[pymethods]
impl ProfileKeyCredentialResponse {
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

#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct ProfileKeyCredential {
    pub state: zkgroup::api::profiles::ProfileKeyCredential,
}

#[pymethods]
impl ProfileKeyCredential {
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

#[pyclass]
#[derive(Copy, Clone, Serialize)]
pub struct ProfileKeyVersion {
    pub state: zkgroup::api::profiles::ProfileKeyVersion,
}

#[pymethods]
impl ProfileKeyVersion {
    fn serialize(&self, py: Python) -> Result<PyObject, ZkGroupError> {
        let bytes = bincode::serialize(&self).expect("could not serialize to bytes");
        Ok(PyBytes::new(py, &bytes).into())
    }
}

#[pymethods]
impl ProfileKey {
    #[staticmethod]
    fn generate(randomness: zkgroup::common::simple_types::RandomnessBytes) -> Self {
        ProfileKey {
            state: zkgroup::api::profiles::ProfileKey::generate(randomness),
        }
    }

    #[staticmethod]
    fn create(bytes: zkgroup::common::simple_types::ProfileKeyBytes) -> Self {
        ProfileKey {
            state: zkgroup::api::profiles::ProfileKey::create(bytes),
        }
    }

    fn get_bytes(&self, py: Python) -> PyObject {
        PyBytes::new(py, &self.state.get_bytes()).into()
    }

    fn get_commitment(
        &self,
        uid_bytes: zkgroup::common::simple_types::UidBytes,
    ) -> ProfileKeyCommitment {
        ProfileKeyCommitment {
            state: self.state.get_commitment(uid_bytes),
        }
    }

    fn get_profile_key_version(
        &self,
        uid_bytes: zkgroup::common::simple_types::UidBytes,
    ) -> ProfileKeyVersion {
        ProfileKeyVersion {
            state: self.state.get_profile_key_version(uid_bytes),
        }
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
impl ProfileKeyCommitment {
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
    module.add_class::<ProfileKey>()?;
    module.add_class::<ProfileKeyCommitment>()?;
    module.add_class::<ProfileKeyCredentialPresentation>()?;
    module.add_class::<ProfileKeyCredentialRequest>()?;
    module.add_class::<ProfileKeyCredentialRequestContext>()?;
    module.add_class::<ProfileKeyCredentialResponse>()?;
    module.add_class::<ProfileKeyCredential>()?;
    module.add_class::<ProfileKeyVersion>()?;
    Ok(())
}
