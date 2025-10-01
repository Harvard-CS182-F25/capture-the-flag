use bevy::prelude::*;
use ctf_core::{
    flag::{CapturePoint, Flag, FlagStatus},
    team::TeamId,
};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pyclass_enum, gen_stub_pymethods};
use serde::{Deserialize, Serialize};

use crate::team::PyTeamId;

/// A snapshot of an flags's state in the game.
#[gen_stub_pyclass]
#[pyclass(name = "FlagState", frozen)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlagState {
    pub id: u32,
    pub name: String,
    pub team: TeamId,
    pub position: (f32, f32),
    pub flag: Flag,
}

#[gen_stub_pymethods]
#[pymethods]
impl FlagState {
    /// The human-readable name of the flag.
    #[getter]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The unique identifier of the flag.
    #[getter]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[getter]
    /// The team the flag belongs to.
    pub fn team(&self) -> PyTeamId {
        PyTeamId { inner: self.team }
    }

    /// The position of the flag in the game world as an (x, y) tuple.
    #[getter]
    pub fn position(&self) -> (f32, f32) {
        self.position
    }

    #[getter]
    pub fn status(&self) -> PyFlagStatus {
        self.flag.status.into()
    }
}

/// A snapshot of an capture point's state in the game.
#[gen_stub_pyclass]
#[pyclass(name = "CapturePointState", frozen)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturePointState {
    name: String,
    id: u32,
    team: TeamId,
    position: (f32, f32),
    has_flag: bool,
}

#[gen_stub_pymethods]
#[pymethods]
impl CapturePointState {
    // The human-readable name of the capture point.
    #[getter]
    fn name(&self) -> &str {
        &self.name
    }

    /// The unique identifier of the capture point.
    #[getter]
    fn id(&self) -> u32 {
        self.id
    }

    #[getter]
    /// The team the capture point belongs to.
    fn team(&self) -> PyTeamId {
        PyTeamId { inner: self.team }
    }

    /// The position of the flag in the game world as an (x, y) tuple.
    #[getter]
    fn position(&self) -> (f32, f32) {
        self.position
    }

    // Whether the capture point currently has a flag.
    fn has_flag(&self) -> bool {
        self.has_flag
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
    flags: Query<(Entity, &Name, &Transform, &Flag)>,
) -> (Vec<FlagState>, Vec<FlagState>) {
    let mut red_team = vec![];
    let mut blue_team = vec![];

    for (entity, name, transform, flag) in &flags {
        let agent_state = FlagState {
            id: entity.index(),
            name: name.as_str().to_string(),
            position: (transform.translation.x, transform.translation.z),
            team: flag.team,
            flag: *flag,
        };

        match flag.team {
            TeamId::Red => red_team.push(agent_state),
            TeamId::Blue => blue_team.push(agent_state),
        }
    }

    red_team.sort_by_key(|a| a.id);
    blue_team.sort_by_key(|a| a.id);

    eprintln!("Red Flags RUST: {:?}", red_team);
    eprintln!("Blue Flags RUST: {:?}", blue_team);

    (red_team, blue_team)
}

pub fn collect_capture_point_states(
    capture_points: Query<(Entity, &Name, &Transform, &CapturePoint)>,
) -> (Vec<CapturePointState>, Vec<CapturePointState>) {
    let mut red_team = vec![];
    let mut blue_team = vec![];

    for (entity, name, transform, capture_point) in &capture_points {
        let cp_state = CapturePointState {
            name: name.as_str().to_string(),
            id: entity.index(),
            team: capture_point.team,
            position: (transform.translation.x, transform.translation.z),
            has_flag: capture_point.flag.is_some(),
        };

        match capture_point.team {
            TeamId::Red => red_team.push(cp_state),
            TeamId::Blue => blue_team.push(cp_state),
        }
    }

    red_team.sort_by_key(|a| a.id);
    blue_team.sort_by_key(|a| a.id);

    (red_team, blue_team)
}
