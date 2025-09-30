use avian3d::prelude::*;
use bevy::prelude::*;

use crate::agent::COLLISION_LAYER_AGENT;
use crate::character_controller::CharacterControllerBundle;
use crate::core::{COLLISION_LAYER_GROUND, CTFConfig};
use crate::interaction_range::VisibleRange;
use crate::team::{Team, TeamId};
use crate::wall::COLLISION_LAYER_WALL;

use super::components::AgentBundle;
use super::visual::AgentGraphicsAssets;

pub fn spawn_agents_headless(mut commands: Commands, config: Res<CTFConfig>) {
    let collision_layer = CollisionLayers::new(
        LayerMask(COLLISION_LAYER_AGENT),
        LayerMask(COLLISION_LAYER_AGENT | COLLISION_LAYER_WALL | COLLISION_LAYER_GROUND),
    );

    for (team, positions) in [
        (TeamId::Red, &config.red_team_agent_positions),
        (TeamId::Blue, &config.blue_team_agent_positions),
    ] {
        for (i, &position) in positions.iter().enumerate() {
            let name = match team {
                TeamId::Blue => format!("Blue Agent {}", i + 1),
                TeamId::Red => format!("Red Agent {}", i + 1),
            };

            commands.spawn((
                AgentBundle {
                    name: Name::new(name),
                    team: Team(team),
                    ..Default::default()
                },
                VisibleRange,
                Transform::from_xyz(position.0, 0.0, position.1),
                CharacterControllerBundle::new(Collider::cuboid(1.0, 1.0, 1.0)),
                Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
                Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
                collision_layer,
            ));
        }
    }
}

pub fn spawn_agents(
    mut commands: Commands,
    graphics: Res<AgentGraphicsAssets>,
    config: Res<CTFConfig>,
) {
    let collision_layer = CollisionLayers::new(
        LayerMask(COLLISION_LAYER_AGENT),
        LayerMask(COLLISION_LAYER_AGENT | COLLISION_LAYER_WALL | COLLISION_LAYER_GROUND),
    );

    for (team, positions) in [
        (TeamId::Red, &config.red_team_agent_positions),
        (TeamId::Blue, &config.blue_team_agent_positions),
    ] {
        for (i, &position) in positions.iter().enumerate() {
            let name = match team {
                TeamId::Blue => format!("Blue Agent {}", i + 1),
                TeamId::Red => format!("Red Agent {}", i + 1),
            };

            let material = match team {
                TeamId::Blue => graphics.blue_material.clone(),
                TeamId::Red => graphics.red_material.clone(),
            };

            commands.spawn((
                AgentBundle {
                    name: Name::new(name),
                    team: Team(team),
                    ..Default::default()
                },
                VisibleRange,
                Mesh3d(graphics.mesh.clone()),
                MeshMaterial3d(material),
                Transform::from_xyz(position.0, 0.0, position.1),
                CharacterControllerBundle::new(Collider::cuboid(1.0, 1.0, 1.0)),
                Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
                Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
                collision_layer,
            ));
        }
    }
}
