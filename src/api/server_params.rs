use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use serde::{Deserialize, Serialize};
use std::convert::TryInto;

use crate::api::{auth, groups, profiles};
use crate::api::errors::ZkGroupError;

use zkgroup;

#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct ServerSecretParams {
    pub state: zkgroup::api::server_params::ServerSecretParams,
}

#[pyclass]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct ServerPublicParams {
    pub state: zkgroup::api::server_params::ServerPublicParams,
}

#[pymethods]
impl ServerSecretParams {
    #[staticmethod]
    fn generate(randomness: zkgroup::common::simple_types::RandomnessBytes) -> Self {
        ServerSecretParams {
            state: zkgroup::api::server_params::ServerSecretParams::generate(randomness),
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

    fn get_public_params(&self) -> ServerPublicParams {
        ServerPublicParams {
            state: self.state.get_public_params(),
        }
    }

    fn sign(
        &self,
        randomness: zkgroup::common::simple_types::RandomnessBytes,
        message: &[u8],
        py: Python,
    ) -> Result<PyObject, ZkGroupError> {
        match self.state.sign(randomness, message) {
            Ok(result) => Ok(PyBytes::new(py, &result).into()),
            Err(err) => Err(ZkGroupError::new(err).into()),
        }
    }

    fn issue_auth_credential(
        &self,
        randomness: zkgroup::common::simple_types::RandomnessBytes,
        uid_bytes: zkgroup::common::simple_types::UidBytes,
        redemption_time: zkgroup::common::simple_types::RedemptionTime,
    ) -> auth::AuthCredentialResponse {
        auth::AuthCredentialResponse {
            state: self
                .state
                .issue_auth_credential(randomness, uid_bytes, redemption_time),
        }
    }

    fn verify_auth_credential_presentation(
        &self,
        group_public_params: groups::GroupPublicParams,
        presentation: &auth::AuthCredentialPresentation,
    ) -> Result<(), ZkGroupError> {
        match self
            .state
            .verify_auth_credential_presentation(group_public_params.state, &presentation.state)
        {
            Ok(_) => Ok(()),
            Err(err) => Err(ZkGroupError::new(err).into()),
        }
    }

    fn verify_profile_key_credential_presentation(
        &self,
        group_public_params: groups::GroupPublicParams,
        presentation: &profiles::ProfileKeyCredentialPresentation,
    ) -> Result<(), ZkGroupError> {
        match self.state.verify_profile_key_credential_presentation(
            group_public_params.state,
            &presentation.state,
        ) {
            Ok(_) => Ok(()),
            Err(err) => Err(ZkGroupError::new(err).into()),
        }
    }

    fn issue_profile_key_credential(
        &self,
        randomness: zkgroup::common::simple_types::RandomnessBytes,
        request: &profiles::ProfileKeyCredentialRequest,
        uid_bytes: zkgroup::common::simple_types::UidBytes,
        commitment: profiles::ProfileKeyCommitment,
    ) -> Result<profiles::ProfileKeyCredentialResponse, ZkGroupError> {
        match self.state.issue_profile_key_credential(
            randomness,
            &request.state,
            uid_bytes,
            commitment.state,
        ) {
            Ok(result) => Ok(profiles::ProfileKeyCredentialResponse { state: result }),
            Err(err) => Err(ZkGroupError::new(err).into()),
        }
    }
}

// TODO: Fix verify_signature - replace signature_1 and signature_2 with a single
// arg of type [u8; 64]
#[pymethods]
impl ServerPublicParams {
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

    fn verify_signature(
        &self,
        message: &[u8],
        signature_1: [u8; 32],
        signature_2: [u8; 32],
    ) -> Result<(), ZkGroupError> {
        let mut signature = Vec::with_capacity(64);
        signature.extend_from_slice(&signature_1);
        signature.extend_from_slice(&signature_2);
        match self
            .state
            .verify_signature(message, signature.try_into().unwrap())
        {
            Ok(_) => Ok(()),
            Err(err) => Err(ZkGroupError::new(err).into()),
        }
    }

    fn receive_auth_credential(
        &self,
        uid_bytes: zkgroup::common::simple_types::UidBytes,
        redemption_time: zkgroup::common::simple_types::RedemptionTime,
        response: &auth::AuthCredentialResponse,
    ) -> Result<auth::AuthCredential, ZkGroupError> {
        match self
            .state
            .receive_auth_credential(uid_bytes, redemption_time, &response.state)
        {
            Ok(state) => Ok(auth::AuthCredential { state }),
            Err(err) => Err(ZkGroupError::new(err).into()),
        }
    }

    fn create_auth_credential_presentation(
        &self,
        randomness: zkgroup::common::simple_types::RandomnessBytes,
        group_secret_params: groups::GroupSecretParams,
        auth_credential: auth::AuthCredential,
    ) -> auth::AuthCredentialPresentation {
        auth::AuthCredentialPresentation {
            state: self.state.create_auth_credential_presentation(
                randomness,
                group_secret_params.state,
                auth_credential.state,
            ),
        }
    }

    fn create_profile_key_credential_request_context(
        &self,
        randomness: zkgroup::common::simple_types::RandomnessBytes,
        uid_bytes: zkgroup::common::simple_types::UidBytes,
        profile_key: profiles::ProfileKey,
    ) -> profiles::ProfileKeyCredentialRequestContext {
        profiles::ProfileKeyCredentialRequestContext {
            state: self.state.create_profile_key_credential_request_context(
                randomness,
                uid_bytes,
                profile_key.state,
            ),
        }
    }

    fn receive_profile_key_credential(
        &self,
        context: &profiles::ProfileKeyCredentialRequestContext,
        response: &profiles::ProfileKeyCredentialResponse,
    ) -> Result<profiles::ProfileKeyCredential, ZkGroupError> {
        match self
            .state
            .receive_profile_key_credential(&context.state, &response.state)
        {
            Ok(state) => Ok(profiles::ProfileKeyCredential { state }),
            Err(err) => Err(ZkGroupError::new(err).into()),
        }
    }

    fn create_profile_key_credential_presentation(
        &self,
        randomness: zkgroup::common::simple_types::RandomnessBytes,
        group_secret_params: groups::GroupSecretParams,
        profile_key_credential: profiles::ProfileKeyCredential,
    ) -> profiles::ProfileKeyCredentialPresentation {
        profiles::ProfileKeyCredentialPresentation {
            state: self.state.create_profile_key_credential_presentation(
                randomness,
                group_secret_params.state,
                profile_key_credential.state,
            ),
        }
    }
}

pub fn init_submodule(module: &PyModule) -> PyResult<()> {
    module.add_class::<ServerSecretParams>()?;
    module.add_class::<ServerPublicParams>()?;
    Ok(())
}
