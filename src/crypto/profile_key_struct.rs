use pyo3::prelude::*;
use pyo3::types::PyBytes;

use crate::common::sho::Sho;
use crate::crypto::uid_struct::UidStruct;

use zkgroup;

//TODO: Default, PartialEq, Serialize, Deserialize
#[pyclass]
#[derive(Copy, Clone)]
pub struct ProfileKeyStruct {
    pub state: zkgroup::crypto::profile_key_struct::ProfileKeyStruct,
}

#[pymethods]
impl ProfileKeyStruct {
    #[new]
    fn new(profile_key_bytes: [u8; 32], uid_bytes: [u8; 16]) -> ProfileKeyStruct {
        ProfileKeyStruct {
            state: zkgroup::crypto::profile_key_struct::ProfileKeyStruct::new(
                profile_key_bytes,
                uid_bytes,
            ),
        }
    }

    // TODO: do we need calc_M3 in public API? (returns RistrettoPoint)

    fn to_bytes(&self, py: Python) -> PyObject {
        PyBytes::new(py, &self.state.to_bytes()).into()
    }
}
