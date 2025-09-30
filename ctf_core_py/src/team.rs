use pyo3::{exceptions::PyValueError, prelude::*};

use ctf_core::team;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use serde::{Deserialize, Serialize};

#[gen_stub_pyclass]
#[pyclass(name = "Team", frozen, eq)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PyTeamId {
    pub inner: team::TeamId,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyTeamId {
    #[classattr]
    const RED: PyTeamId = PyTeamId {
        inner: team::TeamId::Red,
    };

    #[classattr]
    const BLUE: PyTeamId = PyTeamId {
        inner: team::TeamId::Blue,
    };

    pub fn other(&self) -> PyTeamId {
        PyTeamId {
            inner: match self.inner {
                team::TeamId::Red => team::TeamId::Blue,
                team::TeamId::Blue => team::TeamId::Red,
            },
        }
    }

    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        serde_json::from_str(s)
            .map_err(|e| PyValueError::new_err(format!("Invalid team string: {}", e)))
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.inner)
    }
}
