use crate::{
    agent::AGENT_DEFAULT_SPEED,
    team::{Team, TeamId},
};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum Action {
    Move { id: u32, velocity: Vec2 },
}

impl Default for AgentBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Agent"),
            agent: Agent {
                speed: AGENT_DEFAULT_SPEED,
                flag: None,
            },
            team: Team(TeamId::Red),
        }
    }
}
