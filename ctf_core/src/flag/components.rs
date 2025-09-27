use bevy::prelude::*;

use crate::flag::visual::{CapturePointGraphicsAssets, FlagGraphicsAssets};
use crate::interaction_range::{InteractionRadius, VisibleRange};
use crate::team::TeamId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum FlagStatus {
    Dropped,
    PickedUp,
    Captured,
}

#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect)]
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
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
}

impl FlagBundle {
    pub fn new(name: &str, team: TeamId, position: Vec3, graphics: &FlagGraphicsAssets) -> Self {
        Self {
            name: Name::new(name.to_string()),
            flag: Flag {
                team,
                status: FlagStatus::Dropped,
            },
            interaction_radius: InteractionRadius(2.0),
            transform: Transform::from_translation(position),
            visibile_range: VisibleRange,
            mesh: Mesh3d(graphics.mesh.clone()),
            material: match team {
                TeamId::Blue => MeshMaterial3d(graphics.blue_material.clone()),
                TeamId::Red => MeshMaterial3d(graphics.red_material.clone()),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect)]
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
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
}

impl CapturePointBundle {
    pub fn new(
        name: &str,
        team: TeamId,
        position: Vec3,
        graphics: &CapturePointGraphicsAssets,
    ) -> Self {
        Self {
            name: Name::new(name.to_string()),
            capture_point: CapturePoint { team, flag: None },
            interaction_radius: InteractionRadius(1.0),
            transform: Transform::from_translation(position),
            mesh: Mesh3d(graphics.mesh.clone()),
            material: match team {
                TeamId::Red => MeshMaterial3d(graphics.blue_material.clone()),
                TeamId::Blue => MeshMaterial3d(graphics.red_material.clone()),
            },
        }
    }
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource, Default)]
pub struct FlagCaptureCounts {
    pub red: u32,
    pub blue: u32,
}
