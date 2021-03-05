use pyo3::prelude::*;

use crate::crypto::profile_key_struct::ProfileKeyStruct;

use zkgroup;

//TODO: PartialEq, Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
pub struct SystemParams {
    pub state: zkgroup::crypto::profile_key_commitment::SystemParams,
}

//TODO: PartialEq, Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
pub struct CommitmentWithSecretNonce {
    pub state: zkgroup::crypto::profile_key_commitment::CommitmentWithSecretNonce,
}

//TODO: PartialEq, Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
pub struct Commitment {
    pub state: zkgroup::crypto::profile_key_commitment::Commitment,
}

#[pymethods]
impl SystemParams {
    #[staticmethod]
    fn generate() -> SystemParams {
        SystemParams {
            state: zkgroup::crypto::profile_key_commitment::SystemParams::generate(),
        }
    }

    #[staticmethod]
    fn get_hardcoded() -> SystemParams {
        SystemParams {
            state: zkgroup::crypto::profile_key_commitment::SystemParams::get_hardcoded(),
        }
    }
}

#[pymethods]
impl CommitmentWithSecretNonce {
    #[new]
    fn new(
        profile_key: ProfileKeyStruct,
        uid_bytes: zkgroup::common::simple_types::UidBytes,
    ) -> CommitmentWithSecretNonce {
        CommitmentWithSecretNonce {
            state: zkgroup::crypto::profile_key_commitment::CommitmentWithSecretNonce::new(
                profile_key.state,
                uid_bytes,
            ),
        }
    }

    fn get_profile_key_commitment(&self) -> Commitment {
        Commitment {
            state: self.state.get_profile_key_commitment(),
        }
    }

    // TODO: skip calc_j3 as part of public API?
}
