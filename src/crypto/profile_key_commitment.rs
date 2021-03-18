use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use serde::{Deserialize, Serialize};

use crate::crypto::errors::ZkGroupError;
use crate::crypto::profile_key_struct::ProfileKeyStruct;

use zkgroup;

//TODO: PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct SystemParams {
    pub state: zkgroup::crypto::profile_key_commitment::SystemParams,
}

//TODO: PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct CommitmentWithSecretNonce {
    pub state: zkgroup::crypto::profile_key_commitment::CommitmentWithSecretNonce,
}

//TODO: PartialEq
#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Commitment {
    pub state: zkgroup::crypto::profile_key_commitment::Commitment,
}

#[pymethods]
impl SystemParams {
    #[staticmethod]
    fn generate() -> SystemParams {
        SystemParams {
            state: zkgroup::crypto::profile_key_commitment::SystemParams::generate(),
        }
    }

    #[staticmethod]
    fn get_hardcoded() -> SystemParams {
        SystemParams {
            state: zkgroup::crypto::profile_key_commitment::SystemParams::get_hardcoded(),
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
impl CommitmentWithSecretNonce {
    #[new]
    fn new(
        profile_key: ProfileKeyStruct,
        uid_bytes: zkgroup::common::simple_types::UidBytes,
    ) -> CommitmentWithSecretNonce {
        CommitmentWithSecretNonce {
            state: zkgroup::crypto::profile_key_commitment::CommitmentWithSecretNonce::new(
                profile_key.state,
                uid_bytes,
            ),
        }
    }

    fn get_profile_key_commitment(&self) -> Commitment {
        Commitment {
            state: self.state.get_profile_key_commitment(),
        }
    }

    // TODO: skip calc_j3 as part of public API?

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
impl Commitment {
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
    module.add_class::<CommitmentWithSecretNonce>()?;
    module.add_class::<Commitment>()?;
    Ok(())
}
