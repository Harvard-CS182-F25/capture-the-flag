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

#[derive(Debug, Clone, PartialEq, Reflect)]
#[allow(dead_code)]
pub enum Action {
    Move { id: u32, direction: Vec2 },
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
