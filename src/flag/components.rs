use bevy::prelude::*;

use crate::team::TeamId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum FlagStatus {
    AtBase,
    PickedUp,
    Dropped,
    Captured,
}

#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub struct Flag {
    pub team: TeamId,
    pub status: FlagStatus,
}
