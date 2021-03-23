use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use serde::{Deserialize, Serialize};

use zkgroup;

use crate::crypto::errors::ZkGroupError;
use crate::crypto::profile_key_struct::ProfileKeyStruct;
use crate::crypto::sho::Sho;

//TODO: PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct SystemParams {
    pub state: zkgroup::crypto::profile_key_encryption::SystemParams,
}

//TODO: PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct KeyPair {
    pub state: zkgroup::crypto::profile_key_encryption::KeyPair,
}

//TODO: PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct PublicKey {
    pub state: zkgroup::crypto::profile_key_encryption::PublicKey,
}

//TODO: Default, PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Ciphertext {
    pub state: zkgroup::crypto::profile_key_encryption::Ciphertext,
}

#[pymethods]
impl SystemParams {
    #[staticmethod]
    fn generate() -> SystemParams {
        SystemParams {
            state: zkgroup::crypto::profile_key_encryption::SystemParams::generate(),
        }
    }

    #[staticmethod]
    fn get_hardcoded() -> SystemParams {
        SystemParams {
            state: zkgroup::crypto::profile_key_encryption::SystemParams::get_hardcoded(),
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
impl KeyPair {
    #[staticmethod]
    fn derive_from(sho: &mut Sho) -> KeyPair {
        KeyPair {
            state: zkgroup::crypto::profile_key_encryption::KeyPair::derive_from(&mut sho.state),
        }
    }

    fn encrypt(&self, profile_key: ProfileKeyStruct) -> Ciphertext {
        Ciphertext {
            state: self.state.encrypt(profile_key.state),
        }
    }

    fn decrypt(
        &self,
        ciphertext: Ciphertext,
        uid_bytes: zkgroup::common::simple_types::UidBytes,
    ) -> PyResult<ProfileKeyStruct> {
        match self.state.decrypt(ciphertext.state, uid_bytes) {
            Ok(state) => Ok(ProfileKeyStruct { state }),
            Err(err) => Err(ZkGroupError::new(err).into()),
        }
    }

    fn get_public_key(&self) -> PublicKey {
        PublicKey {
            state: self.state.get_public_key(),
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
    module.add_class::<SystemParams>()?;
    module.add_class::<KeyPair>()?;
    module.add_class::<PublicKey>()?;
    module.add_class::<Ciphertext>()?;
    Ok(())
}
