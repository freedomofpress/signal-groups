use pyo3::prelude::*;

use crate::common::sho::Sho;
use crate::crypto::credentials::{BlindedProfileKeyCredential, ProfileKeyCredential};
use crate::crypto::profile_key_struct::ProfileKeyStruct;

use zkgroup;

//TODO:Serialize, Deserialize, PartialEq
#[pyclass]
#[derive(Copy, Clone)]
pub struct KeyPair {
    pub state: zkgroup::crypto::profile_key_credential_request::KeyPair,
}

//TODO:Serialize, Deserialize, PartialEq
#[pyclass]
#[derive(Copy, Clone)]
pub struct PublicKey {
    pub state: zkgroup::crypto::profile_key_credential_request::PublicKey,
}

//TODO:Serialize, Deserialize, PartialEq
#[pyclass]
#[derive(Copy, Clone)]
pub struct CiphertextWithSecretNonce {
    pub state: zkgroup::crypto::profile_key_credential_request::CiphertextWithSecretNonce,
}

//TODO:Serialize, Deserialize, PartialEq
#[pyclass]
#[derive(Copy, Clone)]
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
}

#[pymethods]
impl CiphertextWithSecretNonce {
    fn get_ciphertext(&self) -> Ciphertext {
        Ciphertext {
            state: self.state.get_ciphertext(),
        }
    }
}
