use pyo3::prelude::*;

mod crypto;

/// Signal groups in Python
///
/// This Rust extension provides Python bindings for the Rust crate
/// zkgroups.
#[pymodule]
fn signal_groups(_py: Python, _module: &PyModule) -> PyResult<()> {
    // let crypto_submod = PyModule::new(py, "crypto")?;
    // crypto::init_submodule(crypto_submod)?;
    // module.add_submodule(crypto_submod)?;

    // // Workaround to enable imports from submodules. Upstream issue: pyo3 issue #759
    // // https://github.com/PyO3/pyo3/issues/759#issuecomment-653964601
    // let mods = [
    //     "crypto",
    // ];
    // for module_name in mods.iter() {
    //     let cmd = format!(
    //         "import sys; sys.modules['signal_groups.{}'] = {}",
    //         module_name, module_name
    //     );
    //     py.run(&cmd, None, Some(module.dict()))?;
    // }
    Ok(())
}
