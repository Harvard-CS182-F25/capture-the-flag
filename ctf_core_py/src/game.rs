use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use serde::{Deserialize, Serialize};

use crate::agent::AgentState;
use crate::flag::{CapturePointState, FlagState};
use crate::team::PyTeamId;
use ctf_core::team::TeamId;

/// A snapshot of the current game state, including scores and agent states for both teams.
#[gen_stub_pyclass]
#[pyclass(name = "GameState", frozen)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub red_score: u32,
    pub blue_score: u32,
    pub red_team: Vec<AgentState>,
    pub blue_team: Vec<AgentState>,
    pub red_flags: Vec<FlagState>,
    pub blue_flags: Vec<FlagState>,
    pub num_flags_per_team: u32,
    pub red_capture_points: Vec<CapturePointState>,
    pub blue_capture_points: Vec<CapturePointState>,
}

#[gen_stub_pymethods]
#[pymethods]
impl GameState {
    /// The number of flags captured by the red team.
    #[getter]
    pub fn red_score(&self) -> u32 {
        self.red_score
    }

    /// The number of flags captured by the blue team.
    #[getter]
    pub fn blue_score(&self) -> u32 {
        self.blue_score
    }

    /// The list of agents on the red team, sorted by their IDs.
    #[getter]
    pub fn red_team(&self) -> Vec<AgentState> {
        self.red_team.clone()
    }

    /// The list of agents on the blue team, sorted by their IDs.
    #[getter]
    pub fn blue_team(&self) -> Vec<AgentState> {
        self.blue_team.clone()
    }

    /// The list of flags belonging to the red team, sorted by their IDs.
    #[getter]
    pub fn red_flags(&self) -> Vec<FlagState> {
        self.red_flags.clone()
    }

    /// The list of flags belonging to the blue team, sorted by their IDs.
    #[getter]
    pub fn blue_flags(&self) -> Vec<FlagState> {
        self.blue_flags.clone()
    }

    /// The number of flags each team starts with at the beginning of the game.
    #[getter]
    pub fn num_flags_per_team(&self) -> u32 {
        self.num_flags_per_team
    }

    /// The list of capture points belonging to the red team, sorted by their IDs.
    #[getter]
    pub fn red_capture_points(&self) -> Vec<CapturePointState> {
        self.red_capture_points.clone()
    }

    /// The list of capture points belonging to the blue team, sorted by their IDs.
    #[getter]
    pub fn blue_capture_points(&self) -> Vec<CapturePointState> {
        self.blue_capture_points.clone()
    }

    /// Gets the score for the specified team.
    ///
    /// Parameters
    ///    `team`: The team whose score to retrieve (either `Team.RED` or `Team.BLUE`).
    pub fn get_team_score(&self, team: &PyTeamId) -> u32 {
        match team.inner {
            TeamId::Red => self.red_score,
            TeamId::Blue => self.blue_score,
        }
    }

    /// Gets the list of agents for the specified team.
    ///
    /// Parameters
    ///    `team`: The team whose agents to retrieve (either `Team.RED` or `Team.BLUE`).
    pub fn get_team_agents(&self, team: &PyTeamId) -> Vec<AgentState> {
        match team.inner {
            TeamId::Red => self.red_team.clone(),
            TeamId::Blue => self.blue_team.clone(),
        }
    }

    /// Gets the list of flags for the specified team.
    ///
    /// Parameters
    ///   `team`: The team whose flags to retrieve (either `Team.RED` or `Team.BLUE`).
    pub fn get_team_flags(&self, team: &PyTeamId) -> Vec<FlagState> {
        match team.inner {
            TeamId::Red => self.red_flags.clone(),
            TeamId::Blue => self.blue_flags.clone(),
        }
    }

    /// Gets the list of capture points for the specified team.
    ///
    /// Parameters
    ///  `team`: The team whose capture points to retrieve (either `Team.RED` or `Team.BLUE`).
    pub fn get_team_capture_points(&self, team: &PyTeamId) -> Vec<CapturePointState> {
        match team.inner {
            TeamId::Red => self.red_capture_points.clone(),
            TeamId::Blue => self.blue_capture_points.clone(),
        }
    }

    #[staticmethod]
    pub fn from_json(json_str: &str) -> PyResult<Self> {
        serde_json::from_str(json_str).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to parse GameState from JSON: {}",
                e
            ))
        })
    }
}
