use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use serde::{Deserialize, Serialize};

use zkgroup;

use crate::crypto::errors::ZkGroupError;
use crate::crypto::sho::Sho;
use crate::crypto::uid_struct::UidStruct;
use crate::crypto::{
    credentials, profile_key_commitment, profile_key_credential_request, profile_key_encryption,
    uid_encryption,
};

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct AuthCredentialIssuanceProof {
    pub state: zkgroup::crypto::proofs::AuthCredentialIssuanceProof,
}

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct ProfileKeyCredentialRequestProof {
    pub state: zkgroup::crypto::proofs::ProfileKeyCredentialRequestProof,
}

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct ProfileKeyCredentialIssuanceProof {
    pub state: zkgroup::crypto::proofs::ProfileKeyCredentialIssuanceProof,
}

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct AuthCredentialPresentationProof {
    pub state: zkgroup::crypto::proofs::AuthCredentialPresentationProof,
}

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct ProfileKeyCredentialPresentationProof {
    pub state: zkgroup::crypto::proofs::ProfileKeyCredentialPresentationProof,
}

#[pymethods]
impl AuthCredentialIssuanceProof {
    // TODO: do we need get_poksho_statement in public API?

    #[new]
    fn new(
        key_pair: credentials::KeyPair,
        credential: credentials::AuthCredential,
        uid: UidStruct,
        redemption_time: zkgroup::common::simple_types::RedemptionTime,
        sho: &mut Sho,
    ) -> Self {
        AuthCredentialIssuanceProof {
            state: zkgroup::crypto::proofs::AuthCredentialIssuanceProof::new(
                key_pair.state,
                credential.state,
                uid.state,
                redemption_time,
                &mut sho.state,
            ),
        }
    }

    fn verify(
        &self,
        public_key: credentials::PublicKey,
        credential: credentials::AuthCredential,
        uid_struct: UidStruct,
        redemption_time: zkgroup::common::simple_types::RedemptionTime,
    ) -> Result<(), ZkGroupError> {
        match self.state.verify(
            public_key.state,
            credential.state,
            uid_struct.state,
            redemption_time,
        ) {
            Ok(_) => Ok(()),
            Err(err) => Err(ZkGroupError::new(err).into()),
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
impl ProfileKeyCredentialRequestProof {
    // TODO: do we need get_poksho_statement in public API?

    #[new]
    fn new(
        key_pair: profile_key_credential_request::KeyPair,
        ciphertext: profile_key_credential_request::CiphertextWithSecretNonce,
        commitment: profile_key_commitment::CommitmentWithSecretNonce,
        sho: &mut Sho,
    ) -> Self {
        ProfileKeyCredentialRequestProof {
            state: zkgroup::crypto::proofs::ProfileKeyCredentialRequestProof::new(
                key_pair.state,
                ciphertext.state,
                commitment.state,
                &mut sho.state,
            ),
        }
    }

    fn verify(
        &self,
        public_key: profile_key_credential_request::PublicKey,
        ciphertext: profile_key_credential_request::Ciphertext,
        commitment: profile_key_commitment::Commitment,
    ) -> Result<(), ZkGroupError> {
        match self
            .state
            .verify(public_key.state, ciphertext.state, commitment.state)
        {
            Ok(_) => Ok(()),
            Err(err) => Err(ZkGroupError::new(err).into()),
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
impl ProfileKeyCredentialIssuanceProof {
    // TODO: do we need get_poksho_statement in public API?

    #[new]
    fn new(
        key_pair: credentials::KeyPair,
        request_public_key: profile_key_credential_request::PublicKey,
        request: profile_key_credential_request::Ciphertext,
        blinded_credential: credentials::BlindedProfileKeyCredentialWithSecretNonce,
        uid: UidStruct,
        sho: &mut Sho,
    ) -> Self {
        ProfileKeyCredentialIssuanceProof {
            state: zkgroup::crypto::proofs::ProfileKeyCredentialIssuanceProof::new(
                key_pair.state,
                request_public_key.state,
                request.state,
                blinded_credential.state,
                uid.state,
                &mut sho.state,
            ),
        }
    }

    fn verify(
        &self,
        credentials_public_key: credentials::PublicKey,
        request_public_key: profile_key_credential_request::PublicKey,
        uid_bytes: zkgroup::common::simple_types::UidBytes,
        request: profile_key_credential_request::Ciphertext,
        blinded_credential: credentials::BlindedProfileKeyCredential,
    ) -> Result<(), ZkGroupError> {
        match self.state.verify(
            credentials_public_key.state,
            request_public_key.state,
            uid_bytes,
            request.state,
            blinded_credential.state,
        ) {
            Ok(_) => Ok(()),
            Err(err) => Err(ZkGroupError::new(err).into()),
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
impl AuthCredentialPresentationProof {
    // TODO: do we need get_poksho_statement in public API?

    #[new]
    fn new(
        credentials_public_key: credentials::PublicKey,
        uid_enc_key_pair: uid_encryption::KeyPair,
        credential: credentials::AuthCredential,
        uid: UidStruct,
        uid_ciphertext: uid_encryption::Ciphertext,
        redemption_time: zkgroup::common::simple_types::RedemptionTime,
        sho: &mut Sho,
    ) -> Self {
        AuthCredentialPresentationProof {
            state: zkgroup::crypto::proofs::AuthCredentialPresentationProof::new(
                credentials_public_key.state,
                uid_enc_key_pair.state,
                credential.state,
                uid.state,
                uid_ciphertext.state,
                redemption_time,
                &mut sho.state,
            ),
        }
    }

    fn verify(
        &self,
        credentials_key_pair: credentials::KeyPair,
        uid_enc_public_key: uid_encryption::PublicKey,
        uid_ciphertext: uid_encryption::Ciphertext,
        redemption_time: zkgroup::common::simple_types::RedemptionTime,
    ) -> Result<(), ZkGroupError> {
        match self.state.verify(
            credentials_key_pair.state,
            uid_enc_public_key.state,
            uid_ciphertext.state,
            redemption_time,
        ) {
            Ok(_) => Ok(()),
            Err(err) => Err(ZkGroupError::new(err).into()),
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
impl ProfileKeyCredentialPresentationProof {
    // TODO: do we need get_poksho_statement in public API?

    #[new]
    fn new(
        uid_enc_key_pair: uid_encryption::KeyPair,
        profile_key_enc_key_pair: profile_key_encryption::KeyPair,
        credentials_public_key: credentials::PublicKey,
        credential: credentials::ProfileKeyCredential,
        uid_ciphertext: uid_encryption::Ciphertext,
        profile_key_ciphertext: profile_key_encryption::Ciphertext,
        uid_bytes: zkgroup::common::simple_types::UidBytes,
        profile_key_bytes: zkgroup::common::simple_types::ProfileKeyBytes,
        sho: &mut Sho,
    ) -> Self {
        ProfileKeyCredentialPresentationProof {
            state: zkgroup::crypto::proofs::ProfileKeyCredentialPresentationProof::new(
                uid_enc_key_pair.state,
                profile_key_enc_key_pair.state,
                credentials_public_key.state,
                credential.state,
                uid_ciphertext.state,
                profile_key_ciphertext.state,
                uid_bytes,
                profile_key_bytes,
                &mut sho.state,
            ),
        }
    }

    fn verify(
        &self,
        credentials_key_pair: credentials::KeyPair,
        uid_ciphertext: uid_encryption::Ciphertext,
        uid_enc_public_key: uid_encryption::PublicKey,
        profile_key_ciphertext: profile_key_encryption::Ciphertext,
        profile_key_enc_public_key: profile_key_encryption::PublicKey,
    ) -> Result<(), ZkGroupError> {
        match self.state.verify(
            credentials_key_pair.state,
            uid_ciphertext.state,
            uid_enc_public_key.state,
            profile_key_ciphertext.state,
            profile_key_enc_public_key.state,
        ) {
            Ok(_) => Ok(()),
            Err(err) => Err(ZkGroupError::new(err).into()),
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

pub fn init_submodule(module: &PyModule) -> PyResult<()> {
    module.add_class::<AuthCredentialIssuanceProof>()?;
    module.add_class::<ProfileKeyCredentialRequestProof>()?;
    module.add_class::<ProfileKeyCredentialIssuanceProof>()?;
    module.add_class::<AuthCredentialPresentationProof>()?;
    module.add_class::<ProfileKeyCredentialPresentationProof>()?;
    Ok(())
}
