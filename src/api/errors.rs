use pyo3::create_exception;
use pyo3::prelude::*;
use pyo3::PyErr;

use std::fmt::Write;
use std::{convert, fmt};

use zkgroup;

//pub type Result<T> = std::result::Result<T, ZkGroupException>;

create_exception!(error, ZkGroupException, pyo3::exceptions::PyException);

#[pyclass]
#[derive(Debug)]
pub struct ZkGroupError {
    pub err: zkgroup::common::errors::ZkGroupError,
}

// TODO: Not use debug printing here?
impl fmt::Display for ZkGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.err)
    }
}

impl convert::From<ZkGroupError> for PyErr {
    fn from(err: ZkGroupError) -> Self {
        let mut output = String::new();
        // TODO: handle Err variant here
        write!(&mut output, "{}", err);
        ZkGroupException::new_err(output)
    }
}

impl convert::From<zkgroup::common::errors::ZkGroupError> for ZkGroupError {
    fn from(err: zkgroup::common::errors::ZkGroupError) -> Self {
        ZkGroupError { err }
    }
}

impl ZkGroupError {
    pub fn new(err: zkgroup::common::errors::ZkGroupError) -> Self {
        Self { err }
    }
}

pub fn init_submodule(py: Python, module: &PyModule) -> PyResult<()> {
    module.add("ZkGroupException", py.get_type::<ZkGroupException>())?;
    Ok(())
}
