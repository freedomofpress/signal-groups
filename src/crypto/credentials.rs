use pyo3::prelude::*;

use crate::common::sho::Sho;
use crate::crypto::profile_key_credential_request;
use crate::crypto::uid_struct::UidStruct;

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
    pub state: zkgroup::crypto::credentials::ProfileKeyCredential,
}

// TODO: PartialEq, Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
pub struct BlindedProfileKeyCredentialWithSecretNonce {
    pub state: zkgroup::crypto::credentials::BlindedProfileKeyCredentialWithSecretNonce,
}

// TODO: PartialEq, Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
pub struct BlindedProfileKeyCredential {
    pub state: zkgroup::crypto::credentials::BlindedProfileKeyCredential,
}

impl BlindedProfileKeyCredentialWithSecretNonce {
    pub fn get_blinded_profile_key_credential(&self) -> BlindedProfileKeyCredential {
        BlindedProfileKeyCredential {
            state: self.state.get_blinded_profile_key_credential(),
        }
    }
}
