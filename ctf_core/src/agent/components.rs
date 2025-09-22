use crate::team::{Team, TeamId};
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub struct Agent {
    pub speed: f32,
    pub flag: Option<Entity>,
}

#[derive(Debug, Clone, PartialEq, Bundle)]
pub struct AgentBundle {
    pub name: Name,
    pub agent: Agent,
    pub team: Team,
}

impl Default for AgentBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Agent"),
            agent: Agent {
                speed: 5.0,
                flag: None,
            },
            team: Team(TeamId::Red),
        }
    }
}
