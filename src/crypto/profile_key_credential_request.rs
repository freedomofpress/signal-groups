use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use serde::{Deserialize, Serialize};

use crate::crypto::credentials::{BlindedProfileKeyCredential, ProfileKeyCredential};
use crate::crypto::errors::ZkGroupError;
use crate::crypto::profile_key_struct::ProfileKeyStruct;
use crate::crypto::sho::Sho;

use zkgroup;

//TODO: PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct KeyPair {
    pub state: zkgroup::crypto::profile_key_credential_request::KeyPair,
}

//TODO: PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct PublicKey {
    pub state: zkgroup::crypto::profile_key_credential_request::PublicKey,
}

//TODO: PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct CiphertextWithSecretNonce {
    pub state: zkgroup::crypto::profile_key_credential_request::CiphertextWithSecretNonce,
}

//TODO: PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Ciphertext {
    pub state: zkgroup::crypto::profile_key_credential_request::Ciphertext,
}

#[pymethods]
impl KeyPair {
    #[staticmethod]
    fn generate(sho: &mut Sho) -> KeyPair {
        KeyPair {
            state: zkgroup::crypto::profile_key_credential_request::KeyPair::generate(
                &mut sho.state,
            ),
        }
    }

    fn get_public_key(&self) -> PublicKey {
        PublicKey {
            state: self.state.get_public_key(),
        }
    }

    fn encrypt(
        &self,
        profile_key_struct: ProfileKeyStruct,
        sho: &mut Sho,
    ) -> CiphertextWithSecretNonce {
        CiphertextWithSecretNonce {
            state: self.state.encrypt(profile_key_struct.state, &mut sho.state),
        }
    }

    fn decrypt_blinded_profile_key_credential(
        &self,
        blinded_profile_key_credential: BlindedProfileKeyCredential,
    ) -> ProfileKeyCredential {
        ProfileKeyCredential {
            state: self
                .state
                .decrypt_blinded_profile_key_credential(blinded_profile_key_credential.state),
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

#[pymethods]
impl CiphertextWithSecretNonce {
    fn get_ciphertext(&self) -> Ciphertext {
        Ciphertext {
            state: self.state.get_ciphertext(),
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
impl Ciphertext {
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
    module.add_class::<KeyPair>()?;
    module.add_class::<PublicKey>()?;
    module.add_class::<CiphertextWithSecretNonce>()?;
    module.add_class::<Ciphertext>()?;
    Ok(())
}
