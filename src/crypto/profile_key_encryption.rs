use pyo3::prelude::*;

use zkgroup;

use crate::common::sho::Sho;
use crate::crypto::errors::ZkGroupError;
use crate::crypto::profile_key_struct::ProfileKeyStruct;

//TODO: PartialEq, Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
pub struct SystemParams {
    pub state: zkgroup::crypto::profile_key_encryption::SystemParams,
}

//TODO: PartialEq, Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
pub struct KeyPair {
    pub state: zkgroup::crypto::profile_key_encryption::KeyPair,
}

//TODO: PartialEq, Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
pub struct PublicKey {
    pub state: zkgroup::crypto::profile_key_encryption::PublicKey,
}

//TODO: Default, PartialEq, Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
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
}
