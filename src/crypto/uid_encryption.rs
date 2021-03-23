use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use serde::{Deserialize, Serialize};

use zkgroup;

use crate::crypto::errors::ZkGroupError;
use crate::crypto::sho::Sho;
use crate::crypto::uid_struct::UidStruct;

//TODO: PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct SystemParams {
    pub state: zkgroup::crypto::uid_encryption::SystemParams,
}

//TODO: PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct KeyPair {
    pub state: zkgroup::crypto::uid_encryption::KeyPair,
}

//TODO: PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct PublicKey {
    pub state: zkgroup::crypto::uid_encryption::PublicKey,
}

//TODO: Default, PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Ciphertext {
    pub state: zkgroup::crypto::uid_encryption::Ciphertext,
}

#[pymethods]
impl SystemParams {
    #[staticmethod]
    fn generate() -> SystemParams {
        SystemParams {
            state: zkgroup::crypto::uid_encryption::SystemParams::generate(),
        }
    }

    #[staticmethod]
    fn get_hardcoded() -> SystemParams {
        SystemParams {
            state: zkgroup::crypto::uid_encryption::SystemParams::get_hardcoded(),
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
            state: zkgroup::crypto::uid_encryption::KeyPair::derive_from(&mut sho.state),
        }
    }

    fn encrypt(&self, uid: UidStruct) -> Ciphertext {
        Ciphertext {
            state: self.state.encrypt(uid.state),
        }
    }

    fn decrypt(&self, ciphertext: Ciphertext) -> PyResult<UidStruct> {
        match self.state.decrypt(ciphertext.state) {
            Ok(state) => Ok(UidStruct { state }),
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
