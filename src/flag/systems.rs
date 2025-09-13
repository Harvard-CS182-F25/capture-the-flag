use bevy::prelude::*;

use crate::interaction_range::InteractionRadius;
use crate::team::TeamId;

use super::components::{Flag, FlagStatus};
use super::visual::FlagGraphicsAssets;

pub fn spawn_flags(mut commands: Commands, graphics: Res<FlagGraphicsAssets>) {
    commands.spawn((
        Flag {
            team: TeamId::Blue,
            status: FlagStatus::Dropped,
        },
        InteractionRadius(2.0),
        Mesh3d(graphics.mesh.clone()),
        MeshMaterial3d(graphics.blue_material.clone()),
        Transform::from_xyz(5.0, 0.0, 0.0),
        Name::new("Blue Flag"),
    ));

    commands.spawn((
        Flag {
            team: TeamId::Blue,
            status: FlagStatus::Dropped,
        },
        InteractionRadius(2.0),
        Mesh3d(graphics.mesh.clone()),
        MeshMaterial3d(graphics.blue_material.clone()),
        Transform::from_xyz(-15.0, 0.0, 0.0),
        Name::new("Blue Flag 2"),
    ));

    commands.spawn((
        Flag {
            team: TeamId::Red,
            status: FlagStatus::Dropped,
        },
        InteractionRadius(2.0),
        Mesh3d(graphics.mesh.clone()),
        MeshMaterial3d(graphics.red_material.clone()),
        Transform::from_xyz(0.0, 0.0, 5.0),
        Name::new("Red Flag"),
    ));

    commands.spawn((
        Flag {
            team: TeamId::Red,
            status: FlagStatus::Dropped,
        },
        InteractionRadius(2.0),
        Mesh3d(graphics.mesh.clone()),
        MeshMaterial3d(graphics.red_material.clone()),
        Transform::from_xyz(0.0, 0.0, -15.0),
        Name::new("Red Flag 2"),
    ));
}
