use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use serde::{Deserialize, Serialize};
use std::convert::TryInto;

use zkgroup;

use crate::crypto::errors::ZkGroupError;
use crate::crypto::sho::Sho;

//TODO: PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct KeyPair {
    pub state: zkgroup::crypto::signature::KeyPair,
}

//TODO: PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct PublicKey {
    pub state: zkgroup::crypto::signature::PublicKey,
}

#[pymethods]
impl KeyPair {
    #[staticmethod]
    fn generate(sho: &mut Sho) -> Self {
        KeyPair {
            state: zkgroup::crypto::signature::KeyPair::generate(&mut sho.state),
        }
    }

    fn sign(&self, message: &[u8], sho: &mut Sho, py: Python) -> PyResult<PyObject> {
        match self.state.sign(message, &mut sho.state) {
            Ok(state) => Ok(PyBytes::new(py, &state).into()),
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

    // TODO: fix this - i.e. take [u8; 64] instead of two [u8; 32]
    fn verify(
        &self,
        message: &[u8],
        signature_1: [u8; 32],
        signature_2: [u8; 32],
    ) -> Result<(), ZkGroupError> {
        let mut signature: Vec<u8> = Vec::with_capacity(64);
        signature.extend_from_slice(&signature_1);
        signature.extend_from_slice(&signature_2);
        let signature_res: [u8; 64] = signature.try_into().unwrap();
        Ok(self.state.verify(message, signature_res)?)
    }
}

pub fn init_submodule(module: &PyModule) -> PyResult<()> {
    module.add_class::<KeyPair>()?;
    module.add_class::<PublicKey>()?;
    Ok(())
}
