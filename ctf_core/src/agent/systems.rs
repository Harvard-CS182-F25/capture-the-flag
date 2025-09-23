use avian3d::prelude::*;
use bevy::prelude::*;

use crate::agent::COLLISION_LAYER_AGENT;
use crate::character_controller::CharacterControllerBundle;
use crate::core::COLLISION_LAYER_GROUND;
use crate::interaction_range::VisibleRange;
use crate::team::{Team, TeamId};
use crate::wall::COLLISION_LAYER_WALL;

use super::components::AgentBundle;
use super::visual::AgentGraphicsAssets;

pub fn spawn_agents(mut commands: Commands, graphics: Res<AgentGraphicsAssets>) {
    let collision_layer = CollisionLayers::new(
        LayerMask(COLLISION_LAYER_AGENT),
        LayerMask(COLLISION_LAYER_AGENT | COLLISION_LAYER_WALL | COLLISION_LAYER_GROUND),
    );

    commands.spawn((
        AgentBundle {
            name: Name::new("Blue Agent 1"),
            team: Team(TeamId::Blue),
            ..Default::default()
        },
        VisibleRange,
        Mesh3d(graphics.mesh.clone()),
        MeshMaterial3d(graphics.blue_material.clone()),
        Transform::from_xyz(-30.0, 0.0, 0.0),
        CharacterControllerBundle::new(Collider::cuboid(1.0, 1.0, 1.0)).with_movement(30.0, 1.0),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        collision_layer,
    ));

    commands.spawn((
        AgentBundle {
            name: Name::new("Red Agent 1"),
            team: Team(TeamId::Red),
            ..Default::default()
        },
        VisibleRange,
        Mesh3d(graphics.mesh.clone()),
        MeshMaterial3d(graphics.red_material.clone()),
        Transform::from_xyz(30.0, 0.0, 0.0),
        CharacterControllerBundle::new(Collider::cuboid(1.0, 1.0, 1.0)).with_movement(30.0, 1.0),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        collision_layer,
    ));
}
