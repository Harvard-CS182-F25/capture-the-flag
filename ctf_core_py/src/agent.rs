use bevy::prelude::*;
use ctf_core::{
    agent::{Action, Agent},
    team::{Team, TeamId},
};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

use crate::team::PyTeamId;

/// A snapshot of an agent's state in the game.
#[gen_stub_pyclass]
#[pyclass(name = "AgentState", frozen)]
#[derive(Debug, Clone)]
pub struct AgentState {
    pub name: String,
    pub id: u32,
    pub team: TeamId,
    pub position: (f32, f32),
    pub agent: Agent,
}

#[gen_stub_pymethods]
#[pymethods]
impl AgentState {
    /// The human-readable name of the agent.
    #[getter]
    fn name(&self) -> &str {
        &self.name
    }

    /// The unique identifier of the agent.
    #[getter]
    fn id(&self) -> u32 {
        self.id
    }

    #[getter]
    /// The team the agent belongs to.
    fn team(&self) -> PyTeamId {
        PyTeamId { inner: self.team }
    }

    #[getter]
    /// The maximum speed of the agent.
    fn max_speed(&self) -> f32 {
        self.agent.speed
    }

    /// The position of the agent in the game world as an (x, y) tuple.
    #[getter]
    fn position(&self) -> (f32, f32) {
        self.position
    }

    /// If this agent is currently carrying a flag.
    #[getter]
    fn has_flag(&self) -> bool {
        self.agent.flag.is_some()
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "Action", frozen)]
#[derive(Debug, Clone)]
pub struct PyAction {
    id: u32,
    velocity: (f32, f32),
}

#[gen_stub_pymethods]
#[pymethods]
impl PyAction {
    #[new]
    fn new(id: u32, velocity: (f32, f32)) -> Self {
        PyAction { id, velocity }
    }
}

impl From<PyAction> for Action {
    fn from(val: PyAction) -> Self {
        Action::Move {
            id: val.id,
            velocity: val.velocity.into(),
        }
    }
}

pub fn parse_actions(py: Python, obj: PyObject) -> Vec<Action> {
    let actions: Vec<Py<PyAny>> = match obj.extract(py) {
        Ok(actions) => actions,
        Err(_) => return vec![],
    };

    let mut result = Vec::with_capacity(actions.len());
    for action in actions {
        let action: PyAction = match action.extract(py) {
            Ok(action) => action,
            Err(_) => continue,
        };
        result.push(action.into());
    }
    result
}

pub fn collect_agent_states(
    agents: Query<(Entity, &Name, &Transform, &Agent, &Team)>,
) -> (Vec<AgentState>, Vec<AgentState>) {
    let mut red_team = vec![];
    let mut blue_team = vec![];

    for (entity, name, transform, agent, team) in &agents {
        let agent_state = AgentState {
            name: name.as_str().to_string(),
            id: entity.index(),
            team: team.0,
            position: (transform.translation.x, transform.translation.z),
            agent: *agent,
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
