use pyo3::prelude::*;
use pyo3::types::PyBytes;

use zkgroup;

use crate::common::sho::Sho;
use crate::crypto::errors::ZkGroupError;

//TODO: PartialEq, Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
pub struct KeyPair {
    pub state: zkgroup::crypto::signature::KeyPair,
}

//TODO: PartialEq, Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
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
}

pub fn init_submodule(module: &PyModule) -> PyResult<()> {
    module.add_class::<KeyPair>()?;
    module.add_class::<PublicKey>()?;
    Ok(())
}
