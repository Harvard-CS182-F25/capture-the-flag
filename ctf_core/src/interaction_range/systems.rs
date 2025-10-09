use avian3d::prelude::*;
use bevy::prelude::*;

use crate::agent::{
    AGENT_COOLDOWN_TIME, AGENT_DEFAULT_SPEED, AGENT_FLAG_SPEED, AGENT_TAG_RADIUS, Agent,
    AgentGraphicsAssets,
};
use crate::flag::{
    CapturePoint, FLAG_COOLDOWN_TIME, FLAG_SPAWN_RADIUS, Flag, FlagCaptureCounts, FlagStatus,
};
use crate::interaction_range::RecentlyDropped;
use crate::interaction_range::events::{FlagDropEvent, FlagScoreEvent};
use crate::team::{Team, TeamId};
use crate::{Segment2D, segment_hits_wall_flag_or_capture_point};

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
    mut flags: Query<(&mut Flag, &mut Visibility, &mut Transform), Without<RecentlyDropped>>,
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
        agent.speed = AGENT_FLAG_SPEED;

        commands
            .entity(agent_entity)
            .insert(InteractionRadius(AGENT_TAG_RADIUS));

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
        agent.speed = AGENT_DEFAULT_SPEED;

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

pub fn detect_flag_drop(
    mut writer: EventWriter<FlagDropEvent>,
    carriers: Query<(Entity, &Transform, &Agent, &Team, &InteractionRadius)>,
    agents: Query<(&Transform, &Team), With<Agent>>,
) {
    let red_agents = agents
        .iter()
        .filter(|(_, Team(t))| *t == TeamId::Red)
        .collect::<Vec<_>>();

    let blue_agents = agents
        .iter()
        .filter(|(_, Team(t))| *t == TeamId::Blue)
        .collect::<Vec<_>>();

    for (
        carrier_entity,
        carrier_transform,
        carrier_agent,
        Team(carrier_team),
        InteractionRadius(radius),
    ) in &carriers
    {
        let opposing_agents = match carrier_team {
            TeamId::Red => &blue_agents,
            TeamId::Blue => &red_agents,
        };
        let carrier_pos = carrier_transform.translation.xz();
        for (opponent_transform, _) in opposing_agents {
            let opponent_pos = opponent_transform.translation.xz();
            let dist_sq = carrier_pos.distance_squared(opponent_pos);

            if dist_sq <= radius * radius {
                writer.write(FlagDropEvent {
                    agent: carrier_entity,
                    flag: carrier_agent.flag.expect("carrier must have a flag"),
                });
                break;
            }
        }
    }
}

pub fn handle_flag_drop(
    mut reader: EventReader<FlagDropEvent>,
    mut commands: Commands,
    mut agents: Query<(&mut Agent, &mut LinearVelocity)>,
    mut flags: Query<(&mut Flag, &mut Visibility, &GlobalTransform)>,
    spatial_query: SpatialQuery,
    agent_graphics: Option<Res<AgentGraphicsAssets>>,
) {
    for FlagDropEvent {
        agent: agent_entity,
        flag: flag_entity,
    } in reader.read().copied()
    {
        let Ok((mut agent, mut agent_velocity)) = agents.get_mut(agent_entity) else {
            continue;
        };
        let Ok((mut flag, mut vis, global_tf)) = flags.get_mut(flag_entity) else {
            continue;
        };
        if agent.flag.is_none() || !matches!(flag.status, FlagStatus::PickedUp) {
            continue;
        }

        // visual + status updates
        agent.flag = None;
        commands.entity(agent_entity).remove::<InteractionRadius>();
        if let Some(assets) = agent_graphics.as_ref() {
            let mat = match flag.team {
                TeamId::Red => assets.blue_material.clone(),
                TeamId::Blue => assets.red_material.clone(),
            };
            commands.entity(agent_entity).insert(MeshMaterial3d(mat));
        }
        agent_velocity.0 = Vec3::ZERO;

        commands
            .entity(agent_entity)
            .insert(RecentlyDropped(Timer::from_seconds(
                AGENT_COOLDOWN_TIME,
                TimerMode::Once,
            )));

        flag.status = FlagStatus::Dropped;
        *vis = Visibility::Inherited;
        commands
            .entity(flag_entity)
            .insert(RecentlyDropped(Timer::from_seconds(
                FLAG_COOLDOWN_TIME,
                TimerMode::Once,
            )));

        let mut drop_world = Vec3::ZERO;
        let mut found = false;
        for _ in 0..100 {
            let angle = rand::random::<f32>() * std::f32::consts::TAU;
            let offset = Vec3::new(
                FLAG_SPAWN_RADIUS * angle.cos(),
                0.0,
                FLAG_SPAWN_RADIUS * angle.sin(),
            );
            drop_world = global_tf.translation() + offset;
            drop_world.x = drop_world.x.clamp(-49.0, 49.0);
            drop_world.z = drop_world.z.clamp(-49.0, 49.0);

            if !segment_hits_wall_flag_or_capture_point(
                &spatial_query,
                Segment2D {
                    start: drop_world.xz(),
                    end: drop_world.xz(),
                },
                flag.team,
            ) {
                found = true;
                break;
            }
        }
        if !found {
            drop_world = global_tf.translation();
        }

        commands.queue(move |world: &mut World| {
            let mut e = world.entity_mut(flag_entity);
            e.remove_parent_in_place();
            if let Some(mut t) = e.get_mut::<Transform>() {
                t.translation = drop_world;
            }
        });
    }
}

pub fn tick_recently_dropped(
    mut q: Query<(Entity, &mut RecentlyDropped)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut rd) in &mut q {
        if rd.0.tick(time.delta()).finished() {
            commands.entity(entity).remove::<RecentlyDropped>();
        }
    }
}
