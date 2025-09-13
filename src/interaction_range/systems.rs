use bevy::prelude::*;

use crate::agent::{Agent, AgentGraphicsAssets};
use crate::flag::{Flag, FlagStatus};
use crate::team::{Team, TeamId};

use super::components::{AgentRange, FlagRange, InteractionRadius};
use super::events::FlagPickupEvent;
use super::visual::RingAssets;

const RING_Y_OFFSET: f32 = 0.02; // lift above ground

#[allow(clippy::type_complexity)]
pub fn attach_interaction_range(
    mut commands: Commands,
    ring: Res<RingAssets>,
    interactables: Query<
        (Entity, &InteractionRadius, Option<&Flag>, Option<&Agent>),
        Added<InteractionRadius>,
    >,
) {
    for (entity, InteractionRadius(radius), has_flag, has_agent) in &interactables {
        if has_flag.is_none() && has_agent.is_none() {
            continue;
        }

        let child = commands
            .spawn((
                Name::new("Interaction Range"),
                Mesh3d(ring.mesh.clone()),
                MeshMaterial3d(ring.material.clone()),
                Transform::from_xyz(0.0, RING_Y_OFFSET, 0.0)
                    .with_scale(Vec3::splat(radius.max(1e-4))),
                Visibility::Inherited,
            ))
            .id();

        if has_flag.is_some() {
            commands.entity(child).insert(FlagRange);
        } else if has_agent.is_some() {
            commands.entity(child).insert(AgentRange);
        }

        commands.entity(entity).add_child(child);
    }
}

#[allow(clippy::type_complexity)]
pub fn update_ring_scale_on_radius_change(
    flags: Query<(&InteractionRadius, &Children), Changed<InteractionRadius>>,
    mut ring_transforms: Query<&mut Transform, Or<(With<AgentRange>, With<FlagRange>)>>,
) {
    for (InteractionRadius(r), children) in &flags {
        for child in children.iter() {
            if let Ok(mut t) = ring_transforms.get_mut(child) {
                t.scale = Vec3::splat(r.max(0.0001));
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
            if flag.team == *agent_team
                || !matches!(flag.status, FlagStatus::AtBase | FlagStatus::Dropped)
            {
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
    agent_graphics: Res<AgentGraphicsAssets>,
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

        if agent.flag.is_some() || !matches!(flag.status, FlagStatus::AtBase | FlagStatus::Dropped)
        {
            continue;
        }

        agent.flag = Some(flag_entity);
        let pickup_material = match flag.team {
            TeamId::Red => agent_graphics.blue_pickup_material.clone(),
            TeamId::Blue => agent_graphics.red_pickup_material.clone(),
        };

        commands
            .entity(agent_entity)
            .insert(MeshMaterial3d(pickup_material));

        commands.entity(agent_entity).insert(InteractionRadius(1.0));

        flag.status = FlagStatus::PickedUp;
        commands.entity(flag_entity).insert(ChildOf(agent_entity));
        *flag_visibility = Visibility::Hidden;
        *flag_transform = Transform::IDENTITY;
    }
}
