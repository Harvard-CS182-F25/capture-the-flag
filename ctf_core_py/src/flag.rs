use bevy::prelude::*;
use ctf_core::{
    flag::{Flag, FlagStatus},
    team::{Team, TeamId},
};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pyclass_enum, gen_stub_pymethods};

use crate::team::PyTeamId;

/// A snapshot of an agent's state in the game.
#[gen_stub_pyclass]
#[pyclass(name = "FlagState", frozen)]
#[derive(Debug, Clone)]
pub struct FlagState {
    id: u32,
    name: String,
    team: TeamId,
    position: (f32, f32),
    flag: Flag,
}

#[gen_stub_pymethods]
#[pymethods]
impl FlagState {
    /// The human-readable name of the flag.
    #[getter]
    fn name(&self) -> &str {
        &self.name
    }

    /// The unique identifier of the flag.
    #[getter]
    fn id(&self) -> u32 {
        self.id
    }

    #[getter]
    /// The team the flag belongs to.
    fn team(&self) -> PyTeamId {
        PyTeamId { inner: self.team }
    }

    /// The position of the flag in the game world as an (x, y) tuple.
    #[getter]
    fn position(&self) -> (f32, f32) {
        self.position
    }

    #[getter]
    fn status(&self) -> PyFlagStatus {
        self.flag.status.into()
    }
}

#[gen_stub_pyclass_enum]
#[pyclass(name = "FlagStatus", frozen)]
#[derive(Debug, Clone, Copy)]
pub enum PyFlagStatus {
    Captured,
    PickedUp,
    Dropped,
}

impl From<FlagStatus> for PyFlagStatus {
    fn from(status: FlagStatus) -> Self {
        match status {
            FlagStatus::Captured => PyFlagStatus::Captured,
            FlagStatus::PickedUp => PyFlagStatus::PickedUp,
            FlagStatus::Dropped => PyFlagStatus::Dropped,
        }
    }
}

pub fn collect_flag_states(
    agents: Query<(Entity, &Name, &Transform, &Flag, &Team)>,
) -> (Vec<FlagState>, Vec<FlagState>) {
    let mut red_team = vec![];
    let mut blue_team = vec![];

    for (entity, name, transform, flag, team) in &agents {
        let agent_state = FlagState {
            id: entity.index(),
            name: name.as_str().to_string(),
            position: (transform.translation.x, transform.translation.z),
            team: team.0,
            flag: *flag,
        };

        match team.0 {
            TeamId::Red => red_team.push(agent_state),
            TeamId::Blue => blue_team.push(agent_state),
        }
    }

    red_team.sort_by_key(|a| a.id);
    blue_team.sort_by_key(|a| a.id);

    (red_team, blue_team)
}
