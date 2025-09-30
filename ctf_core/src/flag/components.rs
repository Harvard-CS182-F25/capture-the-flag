use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::interaction_range::{InteractionRadius, VisibleRange};
use crate::team::TeamId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Serialize, Deserialize)]
pub enum FlagStatus {
    Dropped,
    PickedUp,
    Captured,
}

#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Flag {
    pub team: TeamId,
    pub status: FlagStatus,
}

#[derive(Bundle)]
pub struct FlagBundle {
    pub name: Name,
    pub flag: Flag,
    pub interaction_radius: InteractionRadius,
    pub visibile_range: VisibleRange,
    pub transform: Transform,
}

impl FlagBundle {
    pub fn new(name: &str, team: TeamId, position: Vec3) -> Self {
        Self {
            name: Name::new(name.to_string()),
            flag: Flag {
                team,
                status: FlagStatus::Dropped,
            },
            interaction_radius: InteractionRadius(2.0),
            transform: Transform::from_translation(position),
            visibile_range: VisibleRange,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct CapturePoint {
    pub team: TeamId,
    pub flag: Option<Entity>,
}

#[derive(Bundle)]
pub struct CapturePointBundle {
    pub name: Name,
    pub capture_point: CapturePoint,
    pub interaction_radius: InteractionRadius,
    pub transform: Transform,
}

impl CapturePointBundle {
    pub fn new(name: &str, team: TeamId, position: Vec3) -> Self {
        Self {
            name: Name::new(name.to_string()),
            capture_point: CapturePoint { team, flag: None },
            interaction_radius: InteractionRadius(1.0),
            transform: Transform::from_translation(position),
        }
    }
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource, Default)]
pub struct FlagCaptureCounts {
    pub red: u32,
    pub blue: u32,
}
