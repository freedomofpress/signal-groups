use pyo3::prelude::*;

pub mod credentials;

#[pymodule]
fn crypto(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<credentials::SystemParams>()?;
    Ok(())
}

// pub mod profile_key_commitment;
// pub mod profile_key_credential_request;
// pub mod profile_key_encryption;
// pub mod profile_key_struct;
// pub mod proofs;
// pub mod signature;
// pub mod uid_encryption;
// pub mod uid_struct;
