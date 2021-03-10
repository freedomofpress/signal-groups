use pyo3::prelude::*;
use pyo3::types::PyBytes;

// use crate::crypto::profile_key_struct::ProfileKeyStruct;

use crate::api::groups;
use zkgroup;

//TODO: Default, Serialize, Deserialize, PartialEq
#[pyclass]
#[derive(Copy, Clone)]
pub struct ProfileKey {
    pub state: zkgroup::api::profiles::ProfileKey,
}

//TODO: Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
pub struct ProfileKeyCommitment {
    pub state: zkgroup::api::profiles::ProfileKeyCommitment,
}

//TODO: Serialize, Deserialize
#[pyclass]
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
}

//TODO: Serialize, Deserialize
#[pyclass]
pub struct ProfileKeyCredentialRequest {
    pub state: zkgroup::api::profiles::ProfileKeyCredentialRequest,
}

//TODO: Serialize, Deserialize
#[pyclass]
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
}

//TODO: Serialize, Deserialize
#[pyclass]
pub struct ProfileKeyCredentialResponse {
    pub state: zkgroup::api::profiles::ProfileKeyCredentialResponse,
}

//TODO: Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
pub struct ProfileKeyCredential {
    pub state: zkgroup::api::profiles::ProfileKeyCredential,
}

#[pyclass]
#[derive(Copy, Clone)]
pub struct ProfileKeyVersion {
    pub state: zkgroup::api::profiles::ProfileKeyVersion,
}

// TODO: Serialize on ProfileKeyVersion

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
