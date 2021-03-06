use pyo3::prelude::*;

pub mod credentials;
pub mod errors;
pub mod profile_key_commitment;
pub mod profile_key_credential_request;
pub mod profile_key_encryption;
pub mod profile_key_struct;
pub mod proofs;
pub mod signature;
pub mod uid_encryption;
pub mod uid_struct;

#[pymodule]
fn crypto(py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<credentials::SystemParams>()?;
    module.add_class::<credentials::KeyPair>()?;
    module.add_class::<credentials::PublicKey>()?;
    module.add_class::<credentials::AuthCredential>()?;
    module.add_class::<credentials::ProfileKeyCredential>()?;
    module.add_class::<credentials::BlindedProfileKeyCredentialWithSecretNonce>()?;
    module.add_class::<credentials::BlindedProfileKeyCredential>()?;
    module.add_class::<profile_key_credential_request::KeyPair>()?;
    module.add_class::<profile_key_credential_request::PublicKey>()?;
    module.add_class::<profile_key_credential_request::CiphertextWithSecretNonce>()?;
    module.add_class::<profile_key_credential_request::Ciphertext>()?;
    module.add_class::<profile_key_commitment::SystemParams>()?;
    module.add_class::<profile_key_commitment::CommitmentWithSecretNonce>()?;
    module.add_class::<profile_key_commitment::Commitment>()?;
    module.add_class::<profile_key_encryption::SystemParams>()?;
    module.add_class::<profile_key_encryption::KeyPair>()?;
    module.add_class::<profile_key_encryption::PublicKey>()?;
    module.add_class::<profile_key_encryption::Ciphertext>()?;
    module.add_class::<profile_key_struct::ProfileKeyStruct>()?;
    module.add_class::<proofs::AuthCredentialIssuanceProof>()?;
    module.add_class::<proofs::ProfileKeyCredentialRequestProof>()?;
    module.add_class::<proofs::ProfileKeyCredentialIssuanceProof>()?;
    module.add_class::<proofs::AuthCredentialPresentationProof>()?;
    module.add_class::<proofs::ProfileKeyCredentialPresentationProof>()?;
    module.add_class::<signature::KeyPair>()?;
    module.add_class::<signature::PublicKey>()?;
    module.add_class::<uid_encryption::SystemParams>()?;
    module.add_class::<uid_encryption::KeyPair>()?;
    module.add_class::<uid_encryption::PublicKey>()?;
    module.add_class::<uid_encryption::Ciphertext>()?;
    module.add_class::<uid_struct::UidStruct>()?;
    module.add(
        "ZkGroupException",
        py.get_type::<errors::ZkGroupException>(),
    )?;
    Ok(())
}
