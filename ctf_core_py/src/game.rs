use pyo3::prelude::*;

use crate::agent::AgentState;
use crate::team::PyTeamId;
use ctf_core::team::TeamId;

#[pyclass(name = "GameState", frozen)]
#[derive(Debug, Clone)]
pub struct GameState {
    pub red_score: u32,
    pub blue_score: u32,
    pub red_team: Vec<AgentState>,
    pub blue_team: Vec<AgentState>,
}

#[pymethods]
impl GameState {
    #[getter]
    pub fn red_score(&self) -> u32 {
        self.red_score
    }

    #[getter]
    fn blue_score(&self) -> u32 {
        self.blue_score
    }

    #[getter]
    fn red_team(&self) -> Vec<AgentState> {
        self.red_team.clone()
    }

    #[getter]
    fn blue_team(&self) -> Vec<AgentState> {
        self.blue_team.clone()
    }

    fn get_team_score(&self, team: PyTeamId) -> u32 {
        match team.inner {
            TeamId::Red => self.red_score,
            TeamId::Blue => self.blue_score,
        }
    }

    fn get_team_agents(&self, team: PyTeamId) -> Vec<AgentState> {
        match team.inner {
            TeamId::Red => self.red_team.clone(),
            TeamId::Blue => self.blue_team.clone(),
        }
    }
}
