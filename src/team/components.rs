use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum TeamId {
    Red,
    Blue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct Team(pub TeamId);
