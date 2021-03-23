use pyo3::prelude::*;

pub mod credentials;
pub mod errors;
pub mod profile_key_commitment;
pub mod profile_key_credential_request;
pub mod profile_key_encryption;
pub mod profile_key_struct;
pub mod proofs;
pub mod sho;
pub mod signature;
pub mod uid_encryption;
pub mod uid_struct;

#[pymodule]
fn crypto(py: Python, module: &PyModule) -> PyResult<()> {
    let credentials_submod = PyModule::new(py, "credentials")?;
    credentials::init_submodule(credentials_submod)?;
    module.add_submodule(credentials_submod)?;

    let errors_submod = PyModule::new(py, "errors")?;
    errors::init_submodule(py, errors_submod)?;
    module.add_submodule(errors_submod)?;

    let profile_key_credential_request_submod =
        PyModule::new(py, "profile_key_credential_request")?;
    profile_key_credential_request::init_submodule(profile_key_credential_request_submod)?;
    module.add_submodule(profile_key_credential_request_submod)?;

    let profile_key_commitment_submod = PyModule::new(py, "profile_key_commitment")?;
    profile_key_commitment::init_submodule(profile_key_commitment_submod)?;
    module.add_submodule(profile_key_commitment_submod)?;

    let profile_key_encryption_submod = PyModule::new(py, "profile_key_encryption")?;
    profile_key_encryption::init_submodule(profile_key_encryption_submod)?;
    module.add_submodule(profile_key_encryption_submod)?;

    let profile_key_struct_submod = PyModule::new(py, "profile_key_struct")?;
    profile_key_struct::init_submodule(profile_key_struct_submod)?;
    module.add_submodule(profile_key_struct_submod)?;

    let proofs_submod = PyModule::new(py, "proofs")?;
    proofs::init_submodule(proofs_submod)?;
    module.add_submodule(proofs_submod)?;

    let sho_submod = PyModule::new(py, "sho")?;
    sho::init_submodule(sho_submod)?;
    module.add_submodule(sho_submod)?;

    let signature_submod = PyModule::new(py, "signature")?;
    signature::init_submodule(signature_submod)?;
    module.add_submodule(signature_submod)?;

    let uid_encryption_submod = PyModule::new(py, "uid_encryption")?;
    uid_encryption::init_submodule(uid_encryption_submod)?;
    module.add_submodule(uid_encryption_submod)?;

    let uid_struct_submod = PyModule::new(py, "uid_struct")?;
    uid_struct::init_submodule(uid_struct_submod)?;
    module.add_submodule(uid_struct_submod)?;

    // Workaround to enable imports from submodules. Upstream issue: pyo3 issue #759
    // https://github.com/PyO3/pyo3/issues/759#issuecomment-653964601
    let mods = [
        "credentials",
        "errors",
        "profile_key_credential_request",
        "profile_key_commitment",
        "profile_key_encryption",
        "profile_key_struct",
        "proofs",
        "sho",
        "signature",
        "uid_encryption",
        "uid_struct",
    ];
    for module_name in mods.iter() {
        let cmd = format!(
            "import sys; sys.modules['signal_groups.crypto.{}'] = {}",
            module_name, module_name
        );
        py.run(&cmd, None, Some(module.dict()))?;
    }

    Ok(())
}
