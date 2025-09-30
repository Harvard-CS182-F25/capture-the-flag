use bevy::prelude::*;

use crate::agent::{Agent, AgentGraphicsAssets};
use crate::flag::{CapturePoint, Flag, FlagCaptureCounts, FlagStatus};
use crate::interaction_range::events::FlagScoreEvent;
use crate::team::{Team, TeamId};

use super::components::{InteractionRadius, InteractionRange, VisibleRange};
use super::events::FlagPickupEvent;
use super::visual::RingAssets;

const RING_Y_OFFSET: f32 = 0.02; // lift above ground

#[allow(clippy::type_complexity)]
pub fn attach_interaction_range(
    mut commands: Commands,
    ring: Res<RingAssets>,
    interactables: Query<
        (Entity, &InteractionRadius),
        (Added<InteractionRadius>, With<VisibleRange>),
    >,
) {
    for (entity, InteractionRadius(radius)) in &interactables {
        let child = commands
            .spawn((
                Name::new("Interaction Range"),
                InteractionRange,
                Mesh3d(ring.mesh.clone()),
                MeshMaterial3d(ring.material.clone()),
                Transform::from_xyz(0.0, RING_Y_OFFSET, 0.0)
                    .with_scale(Vec3::splat(radius.max(1e-4))),
                Visibility::Inherited,
            ))
            .id();

        commands.entity(entity).add_child(child);
    }
}

#[allow(clippy::type_complexity)]
pub fn update_ring_scale_on_radius_change(
    flags: Query<(&InteractionRadius, &Children), Changed<InteractionRadius>>,
    mut ring_transforms: Query<&mut Transform, With<InteractionRange>>,
) {
    for (InteractionRadius(r), children) in &flags {
        for child in children.iter() {
            if let Ok(mut t) = ring_transforms.get_mut(child) {
                t.scale = Vec3::splat(r.max(0.0001));
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn remove_ring_on_radius_removal(
    mut commands: Commands,
    mut removed: RemovedComponents<InteractionRadius>,
    children: Query<&Children>,
    range_marker: Query<(), With<InteractionRange>>,
) {
    for parent in removed.read() {
        if let Ok(children) = children.get(parent) {
            for child in children.iter() {
                if range_marker.get(child).is_ok() {
                    commands.entity(child).despawn();
                }
            }
        }
    }
}

pub fn detect_flag_pickups(
    mut writer: EventWriter<FlagPickupEvent>,
    agents: Query<(Entity, &Transform, &Team, &Agent)>,
    flags: Query<(Entity, &Transform, &Flag, &InteractionRadius)>,
) {
    for (agent_entity, agent_transform, Team(agent_team), agent) in &agents {
        if agent.flag.is_some() {
            // already carrying a flag
            continue;
        }
        let agent_pos = agent_transform.translation.xz();
        for (flag_entity, flag_transform, flag, InteractionRadius(radius)) in &flags {
            if flag.team == *agent_team || !matches!(flag.status, FlagStatus::Dropped) {
                // can't pick up own flag, or flag that is not on the ground
                continue;
            }
            let flag_pos = flag_transform.translation.xz();
            let dist_sq = agent_pos.distance_squared(flag_pos);
            if dist_sq < radius * radius {
                writer.write(FlagPickupEvent {
                    agent: agent_entity,
                    flag: flag_entity,
                });
                // only pick up one flag at a time
                break;
            }
        }
    }
}

pub fn handle_flag_pickups(
    mut commands: Commands,
    mut reader: EventReader<FlagPickupEvent>,
    mut agents: Query<&mut Agent>,
    mut flags: Query<(&mut Flag, &mut Visibility, &mut Transform)>,
    agent_graphics: Option<Res<AgentGraphicsAssets>>,
) {
    for FlagPickupEvent {
        agent: agent_entity,
        flag: flag_entity,
    } in reader.read().copied()
    {
        let Ok(mut agent) = agents.get_mut(agent_entity) else {
            continue;
        };

        let Ok((mut flag, mut flag_visibility, mut flag_transform)) = flags.get_mut(flag_entity)
        else {
            continue;
        };

        if agent.flag.is_some() || !matches!(flag.status, FlagStatus::Dropped) {
            continue;
        }

        agent.flag = Some(flag_entity);
        if let Some(agent_graphics) = agent_graphics.as_ref() {
            let pickup_material = match flag.team {
                TeamId::Red => agent_graphics.blue_pickup_material.clone(),
                TeamId::Blue => agent_graphics.red_pickup_material.clone(),
            };
            commands
                .entity(agent_entity)
                .insert(MeshMaterial3d(pickup_material));
        }

        commands.entity(agent_entity).insert(InteractionRadius(1.0));

        flag.status = FlagStatus::PickedUp;
        commands.entity(flag_entity).insert(ChildOf(agent_entity));
        *flag_visibility = Visibility::Hidden;
        *flag_transform = Transform::IDENTITY;
    }
}

pub fn detect_flag_capture(
    mut writer: EventWriter<FlagScoreEvent>,
    agents: Query<(Entity, &Transform, &Team, &Agent)>,
    capture_points: Query<(Entity, &InteractionRadius, &Transform, &CapturePoint)>,
) {
    for (agent_entity, agent_transform, Team(agent_team), agent) in &agents {
        if agent.flag.is_none() {
            // not carrying a flag
            continue;
        }

        let agent_pos = agent_transform.translation.xz();
        for (
            capture_point_entity,
            InteractionRadius(radius),
            capture_point_transform,
            capture_point,
        ) in &capture_points
        {
            if capture_point.team != *agent_team || capture_point.flag.is_some() {
                // can only capture at your own point and if the point doesn't have a flag
                continue;
            }
            let capture_point_pos = capture_point_transform.translation.xz();
            let dist_sq = agent_pos.distance_squared(capture_point_pos);
            if dist_sq < radius * radius {
                writer.write(FlagScoreEvent {
                    agent: agent_entity,
                    capture_point: capture_point_entity,
                });
                break;
            }
        }
    }
}

pub fn handle_flag_capture(
    mut commands: Commands,
    mut reader: EventReader<FlagScoreEvent>,
    mut agents: Query<&mut Agent>,
    mut flags: Query<(&mut Flag, &mut Visibility, &mut Transform)>,
    mut capture_points: Query<&mut CapturePoint>,
    mut capture_counts: ResMut<FlagCaptureCounts>,
    agent_graphics: Option<Res<AgentGraphicsAssets>>,
) {
    for FlagScoreEvent {
        agent: agent_entity,
        capture_point: capture_point_entity,
    } in reader.read().copied()
    {
        let Ok(mut agent) = agents.get_mut(agent_entity) else {
            continue;
        };

        let Some(flag_entity) = agent.flag else {
            continue;
        };

        let Ok((mut flag, mut flag_visibility, mut flag_transform)) = flags.get_mut(flag_entity)
        else {
            continue;
        };

        let Ok(mut capture_point) = capture_points.get_mut(capture_point_entity) else {
            continue;
        };

        if flag.team == capture_point.team || !matches!(flag.status, FlagStatus::PickedUp) {
            // can't drop off at own point, or if the flag is not picked up
            continue;
        }

        agent.flag = None;
        commands.entity(agent_entity).remove::<InteractionRadius>();

        if let Some(agent_graphics) = agent_graphics.as_ref() {
            let default_material = match flag.team {
                TeamId::Red => agent_graphics.blue_material.clone(),
                TeamId::Blue => agent_graphics.red_material.clone(),
            };
            commands
                .entity(agent_entity)
                .insert(MeshMaterial3d(default_material));
        }

        flag.status = FlagStatus::Captured;
        capture_point.flag = Some(flag_entity);
        commands.entity(flag_entity).remove::<ChildOf>();
        commands.entity(flag_entity).remove::<InteractionRadius>();
        commands
            .entity(flag_entity)
            .insert(ChildOf(capture_point_entity));
        *flag_visibility = Visibility::Inherited;
        *flag_transform = Transform::IDENTITY;

        match flag.team {
            TeamId::Red => capture_counts.blue += 1,
            TeamId::Blue => capture_counts.red += 1,
        }
    }
}
