use pyo3::prelude::*;
use pyo3::types::PyBytes;

use crate::api::profiles;
use crate::crypto;
use crate::crypto::errors::ZkGroupError;

use zkgroup;

//TODO: Default, Serialize, Deserialize, PartialEq
#[pyclass]
#[derive(Copy, Clone)]
pub struct UuidCiphertext {
    pub state: zkgroup::api::groups::UuidCiphertext,
}

//TODO: Default, Serialize, Deserialize, PartialEq
#[pyclass]
#[derive(Copy, Clone)]
pub struct ProfileKeyCiphertext {
    pub state: zkgroup::api::groups::ProfileKeyCiphertext,
}

//TODO: Default, Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
pub struct GroupMasterKey {
    pub state: zkgroup::api::groups::GroupMasterKey,
}

//TODO: Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
pub struct GroupSecretParams {
    pub state: zkgroup::api::groups::GroupSecretParams,
}

//TODO: Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
pub struct GroupPublicParams {
    pub state: zkgroup::api::groups::GroupPublicParams,
}

#[pymethods]
impl GroupMasterKey {
    #[new]
    fn new(bytes: zkgroup::common::simple_types::GroupMasterKeyBytes) -> Self {
        GroupMasterKey {
            state: zkgroup::api::groups::GroupMasterKey::new(bytes),
        }
    }
}

#[pymethods]
impl GroupSecretParams {
    #[staticmethod]
    fn generate(randomness: zkgroup::common::simple_types::RandomnessBytes) -> Self {
        GroupSecretParams {
            state: zkgroup::api::groups::GroupSecretParams::generate(randomness),
        }
    }

    #[staticmethod]
    fn derive_from_master_key(master_key: GroupMasterKey) -> Self {
        GroupSecretParams {
            state: zkgroup::api::groups::GroupSecretParams::derive_from_master_key(
                master_key.state,
            ),
        }
    }

    fn get_master_key(&self) -> GroupMasterKey {
        GroupMasterKey {
            state: self.state.get_master_key(),
        }
    }

    fn get_group_identifier(&self) -> zkgroup::common::simple_types::GroupIdentifierBytes {
        self.state.get_group_identifier()
    }

    fn get_public_params(&self) -> GroupPublicParams {
        GroupPublicParams {
            state: self.state.get_public_params(),
        }
    }

    fn encrypt_uuid(&self, uid_bytes: zkgroup::common::simple_types::UidBytes) -> UuidCiphertext {
        UuidCiphertext {
            state: self.state.encrypt_uuid(uid_bytes),
        }
    }

    fn encrypt_uid_struct(&self, uid: crypto::uid_struct::UidStruct) -> UuidCiphertext {
        UuidCiphertext {
            state: self.state.encrypt_uid_struct(uid.state),
        }
    }

    fn decrypt_uuid(
        &self,
        ciphertext: UuidCiphertext,
    ) -> Result<zkgroup::common::simple_types::UidBytes, ZkGroupError> {
        match self.state.decrypt_uuid(ciphertext.state) {
            Ok(result) => Ok(result),
            Err(err) => Err(ZkGroupError::new(err).into()),
        }
    }

    fn encrypt_profile_key(
        &self,
        profile_key: profiles::ProfileKey,
        uid_bytes: zkgroup::common::simple_types::UidBytes,
    ) -> ProfileKeyCiphertext {
        ProfileKeyCiphertext {
            state: self.state.encrypt_profile_key(profile_key.state, uid_bytes),
        }
    }

    fn encrypt_profile_key_bytes(
        &self,
        profile_key_bytes: zkgroup::common::simple_types::ProfileKeyBytes,
        uid_bytes: zkgroup::common::simple_types::UidBytes,
    ) -> ProfileKeyCiphertext {
        ProfileKeyCiphertext {
            state: self
                .state
                .encrypt_profile_key_bytes(profile_key_bytes, uid_bytes),
        }
    }

    fn decrypt_profile_key(
        &self,
        ciphertext: ProfileKeyCiphertext,
        uid_bytes: zkgroup::common::simple_types::UidBytes,
    ) -> Result<profiles::ProfileKey, ZkGroupError> {
        match self.state.decrypt_profile_key(ciphertext.state, uid_bytes) {
            Ok(result) => Ok(profiles::ProfileKey { state: result }),
            Err(err) => Err(ZkGroupError::new(err).into()),
        }
    }

    fn encrypt_blob(
        &self,
        randomness: zkgroup::common::simple_types::RandomnessBytes,
        plaintext: &[u8],
        py: Python,
    ) -> Result<PyObject, ZkGroupError> {
        match self.state.encrypt_blob(randomness, plaintext) {
            Ok(result) => Ok(PyBytes::new(py, &result).into()),
            Err(err) => Err(ZkGroupError::new(err).into()),
        }
    }

    fn decrypt_blob(&self, ciphertext: &[u8], py: Python) -> Result<PyObject, ZkGroupError> {
        match self.state.decrypt_blob(ciphertext) {
            Ok(result) => Ok(PyBytes::new(py, &result).into()),
            Err(err) => Err(ZkGroupError::new(err).into()),
        }
    }
}

#[pymethods]
impl GroupPublicParams {
    fn get_group_identifier(&self) -> zkgroup::common::simple_types::GroupIdentifierBytes {
        self.state.get_group_identifier()
    }
}

pub fn init_submodule(module: &PyModule) -> PyResult<()> {
    module.add_class::<ProfileKeyCiphertext>()?;
    module.add_class::<UuidCiphertext>()?;
    module.add_class::<GroupMasterKey>()?;
    module.add_class::<GroupSecretParams>()?;
    module.add_class::<GroupPublicParams>()?;
    Ok(())
}
