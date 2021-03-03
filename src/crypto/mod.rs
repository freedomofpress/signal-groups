use pyo3::prelude::*;

pub mod credentials;
pub mod profile_key_commitment;
pub mod profile_key_credential_request;
pub mod profile_key_encryption;
pub mod profile_key_struct;
pub mod proofs;
pub mod signature;
pub mod uid_encryption;
pub mod uid_struct;

#[pymodule]
fn crypto(_py: Python, module: &PyModule) -> PyResult<()> {
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
    module.add_class::<uid_struct::UidStruct>()?;
    module.add_class::<profile_key_struct::ProfileKeyStruct>()?;
    Ok(())
}
