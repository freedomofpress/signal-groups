use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use serde::{Deserialize, Serialize};

use crate::crypto::errors::ZkGroupError;
use crate::crypto::profile_key_credential_request;
use crate::crypto::sho::Sho;
use crate::crypto::uid_struct::UidStruct;

use zkgroup;

//TODO: Default, PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct SystemParams {
    pub state: zkgroup::crypto::credentials::SystemParams,
}

#[pymethods]
impl SystemParams {
    #[staticmethod]
    fn generate() -> SystemParams {
        SystemParams {
            state: zkgroup::crypto::credentials::SystemParams::generate(),
        }
    }

    #[staticmethod]
    fn get_hardcoded() -> SystemParams {
        SystemParams {
            state: zkgroup::crypto::credentials::SystemParams::get_hardcoded(),
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

// TODO: PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct KeyPair {
    pub state: zkgroup::crypto::credentials::KeyPair,
}

#[pymethods]
impl KeyPair {
    #[staticmethod]
    fn generate(sho: &mut Sho, num_attributes: usize) -> Self {
        KeyPair {
            state: zkgroup::crypto::credentials::KeyPair::generate(&mut sho.state, num_attributes),
        }
    }

    fn get_public_key(&self) -> PublicKey {
        PublicKey {
            state: self.state.get_public_key(),
        }
    }

    fn create_auth_credential(
        &self,
        uid: UidStruct,
        redemption_time: u32,
        sho: &mut Sho,
    ) -> AuthCredential {
        AuthCredential {
            state: self
                .state
                .create_auth_credential(uid.state, redemption_time, &mut sho.state),
        }
    }

    fn create_blinded_profile_key_credential(
        &self,
        uid: UidStruct,
        public_key: profile_key_credential_request::PublicKey,
        ciphertext: profile_key_credential_request::Ciphertext,
        sho: &mut Sho,
    ) -> BlindedProfileKeyCredentialWithSecretNonce {
        BlindedProfileKeyCredentialWithSecretNonce {
            state: self.state.create_blinded_profile_key_credential(
                uid.state,
                public_key.state,
                ciphertext.state,
                &mut sho.state,
            ),
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

// TODO: PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct PublicKey {
    pub state: zkgroup::crypto::credentials::PublicKey,
}

#[pymethods]
impl PublicKey {
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

// TODO: PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct AuthCredential {
    pub state: zkgroup::crypto::credentials::AuthCredential,
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

// TODO: PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct ProfileKeyCredential {
    pub state: zkgroup::crypto::credentials::ProfileKeyCredential,
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

// TODO: PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct BlindedProfileKeyCredentialWithSecretNonce {
    pub state: zkgroup::crypto::credentials::BlindedProfileKeyCredentialWithSecretNonce,
}

// TODO: PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct BlindedProfileKeyCredential {
    pub state: zkgroup::crypto::credentials::BlindedProfileKeyCredential,
}

#[pymethods]
impl BlindedProfileKeyCredential {
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
impl BlindedProfileKeyCredentialWithSecretNonce {
    pub fn get_blinded_profile_key_credential(&self) -> BlindedProfileKeyCredential {
        BlindedProfileKeyCredential {
            state: self.state.get_blinded_profile_key_credential(),
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

pub fn init_submodule(module: &PyModule) -> PyResult<()> {
    module.add_class::<SystemParams>()?;
    module.add_class::<KeyPair>()?;
    module.add_class::<PublicKey>()?;
    module.add_class::<AuthCredential>()?;
    module.add_class::<ProfileKeyCredential>()?;
    module.add_class::<BlindedProfileKeyCredentialWithSecretNonce>()?;
    module.add_class::<BlindedProfileKeyCredential>()?;
    Ok(())
}
