use avian3d::prelude::*;
use bevy::prelude::*;

use crate::character_controller::CharacterControllerBundle;
use crate::team::{Team, TeamId};

use super::components::AgentBundle;
use super::visual::AgentGraphicsAssets;

pub fn spawn_agents(mut commands: Commands, graphics: Res<AgentGraphicsAssets>) {
    commands.spawn((
        AgentBundle {
            name: Name::new("Agent"),
            team: Team(TeamId::Blue),
            ..Default::default()
        },
        Mesh3d(graphics.mesh.clone()),
        MeshMaterial3d(graphics.blue_material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        CharacterControllerBundle::new(Collider::cuboid(1.0, 1.0, 1.0)).with_movement(30.0, 0.92),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
    ));
}
