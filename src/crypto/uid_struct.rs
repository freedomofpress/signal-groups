use pyo3::prelude::*;
use pyo3::types::PyBytes;

use zkgroup;

//TODO:Serialize, Deserialize, PartialEq
#[pyclass]
#[derive(Copy, Clone)]
pub struct UidStruct {
    pub state: zkgroup::crypto::uid_struct::UidStruct,
}

#[pymethods]
impl UidStruct {
    #[new]
    fn new(uid_bytes: zkgroup::common::simple_types::UidBytes) -> UidStruct {
        UidStruct {
            state: zkgroup::crypto::uid_struct::UidStruct::new(uid_bytes),
        }
    }

    // TODO: do we need from_M2 in public API?

    pub fn to_bytes(&self, py: Python) -> PyObject {
        PyBytes::new(py, &self.state.to_bytes()).into()
    }
}
