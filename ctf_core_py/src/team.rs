use pyo3::prelude::*;

use ctf_core::team;

#[pyclass(name = "Team", frozen)]
#[derive(Debug, Clone)]
pub struct PyTeamId {
    pub inner: team::TeamId,
}

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
