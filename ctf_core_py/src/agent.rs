use bevy::prelude::*;
use ctf_core::{
    agent::{Action, Agent},
    team::{Team, TeamId},
};
use pyo3::prelude::*;

#[pyclass(name = "AgentState", frozen)]
#[derive(Debug, Clone)]
pub struct AgentState {
    pub name: String,
    pub id: u32,
    pub team: TeamId,
    pub position: (f32, f32),
    pub has_flag: bool,
}

#[pymethods]
impl AgentState {
    #[getter]
    fn name(&self) -> &str {
        &self.name
    }

    #[getter]
    fn id(&self) -> u32 {
        self.id
    }

    #[getter]
    fn position(&self) -> (f32, f32) {
        self.position
    }

    #[getter]
    fn has_flag(&self) -> bool {
        self.has_flag
    }
}

#[pyclass(name = "Action", frozen)]
#[derive(Debug, Clone)]
pub enum PyAction {
    Move { id: u32, velocity: (f32, f32) },
}

impl From<PyAction> for Action {
    fn from(val: PyAction) -> Self {
        match val {
            PyAction::Move { id, velocity } => Action::Move {
                id,
                velocity: velocity.into(),
            },
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
            has_flag: agent.flag.is_some(),
        };

        match team.0 {
            TeamId::Red => red_team.push(agent_state),
            TeamId::Blue => blue_team.push(agent_state),
        }
    }

    (red_team, blue_team)
}
