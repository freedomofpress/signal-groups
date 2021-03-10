use pyo3::prelude::*;

use crate::api::groups;

use zkgroup;

//TODO: Serialize, Deserialize
#[pyclass]
pub struct AuthCredentialPresentation {
    pub state: zkgroup::api::auth::AuthCredentialPresentation,
}

//TODO: Serialize, Deserialize
#[pyclass]
pub struct AuthCredentialResponse {
    pub state: zkgroup::api::auth::AuthCredentialResponse,
}

//TODO: Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
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
}

pub fn init_submodule(module: &PyModule) -> PyResult<()> {
    module.add_class::<AuthCredential>()?;
    module.add_class::<AuthCredentialResponse>()?;
    module.add_class::<AuthCredentialPresentation>()?;
    Ok(())
}
