use pyo3::prelude::*;

pub mod auth;
pub mod errors;
pub mod groups;
pub mod profiles;
pub mod server_params;

#[pymodule]
fn api(py: Python, module: &PyModule) -> PyResult<()> {
    let auth_submod = PyModule::new(py, "auth")?;
    auth::init_submodule(auth_submod)?;
    module.add_submodule(auth_submod)?;

    let errors_submod = PyModule::new(py, "errors")?;
    errors::init_submodule(py, errors_submod)?;
    module.add_submodule(errors_submod)?;

    let groups_submod = PyModule::new(py, "groups")?;
    groups::init_submodule(groups_submod)?;
    module.add_submodule(groups_submod)?;

    let profiles_submod = PyModule::new(py, "profiles")?;
    profiles::init_submodule(profiles_submod)?;
    module.add_submodule(profiles_submod)?;

    let server_params_submod = PyModule::new(py, "server_params")?;
    server_params::init_submodule(server_params_submod)?;
    module.add_submodule(server_params_submod)?;

    // Workaround to enable imports from submodules. Upstream issue: pyo3 issue #759
    // https://github.com/PyO3/pyo3/issues/759#issuecomment-653964601
    let mods = ["auth", "errors", "groups", "profiles", "server_params"];
    for module_name in mods.iter() {
        let cmd = format!(
            "import sys; sys.modules['signal_groups.api.{}'] = {}",
            module_name, module_name
        );
        py.run(&cmd, None, Some(module.dict()))?;
    }

    Ok(())
}
