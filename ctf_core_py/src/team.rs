use pyo3::prelude::*;

use ctf_core::team;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[gen_stub_pyclass]
#[pyclass(name = "Team", frozen)]
#[derive(Debug, Clone)]
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
}
