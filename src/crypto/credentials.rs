use pyo3::prelude::*;

use crate::common::sho::Sho;

use zkgroup;

//TODO:Serialize, Deserialize, Default, PartialEq
#[pyclass]
#[derive(Copy, Clone)]
pub struct SystemParams {
    pub state: zkgroup::crypto::credentials::SystemParams,
}

#[pymethods]
impl SystemParams {
    #[staticmethod]
    fn generate() -> SystemParams {
        SystemParams{
            state: zkgroup::crypto::credentials::SystemParams::generate()
        }
    }

    #[staticmethod]
    fn get_hardcoded() -> SystemParams {
        SystemParams{
            state: zkgroup::crypto::credentials::SystemParams::get_hardcoded()
        }
    }
}

// TODO: PartialEq, Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
pub struct KeyPair {
    pub state: zkgroup::crypto::credentials::KeyPair,
}

#[pymethods]
impl KeyPair {
    #[staticmethod]
    fn generate(sho: &mut Sho, num_attributes: usize) -> Self {
        KeyPair{
            state: zkgroup::crypto::credentials::KeyPair::generate(&mut sho.state, num_attributes)
        }
    }

    fn get_public_key(&self) -> PublicKey {
        PublicKey{
            state: self.state.get_public_key()
        }
    }
}

// TODO: PartialEq, Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
pub struct PublicKey {
    pub state: zkgroup::crypto::credentials::PublicKey,
}

// TODO: PartialEq, Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
pub struct AuthCredential {
    pub state: zkgroup::crypto::credentials::AuthCredential,
}

// TODO: PartialEq, Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
pub struct ProfileKeyCredential {
    pub state: zkgroup::crypto::credentials::AuthCredential,
}

// TODO: PartialEq, Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
pub struct BlindedProfileKeyCredentialWithSecretNonce {
    pub state: zkgroup::crypto::credentials::AuthCredential,
}

// TODO: PartialEq, Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
pub struct BlindedProfileKeyCredential {
    pub state: zkgroup::crypto::credentials::AuthCredential,
}

