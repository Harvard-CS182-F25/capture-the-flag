use avian3d::prelude::*;
use bevy::prelude::*;

use crate::interaction_range::RecentlyDropped;

use super::components::{CharacterController, Grounded};
use super::events::MovementEvent;

pub fn update_grounded(
    mut commands: Commands,
    mut query: Query<(Entity, &ShapeHits), With<CharacterController>>,
) {
    for (entity, hits) in &mut query {
        let is_grounded = !hits.is_empty();
        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn movement(
    mut movement_event_reader: EventReader<MovementEvent>,
    mut controllers: Query<(
        Entity,
        &mut LinearVelocity,
        &mut AngularVelocity,
        Option<&RecentlyDropped>,
        Has<Grounded>,
    )>,
) {
    for event in movement_event_reader.read() {
        for (entity, mut linear_velocity, mut angular_velocity, recently_tagged, is_grounded) in
            &mut controllers
        {
            if recently_tagged.is_some() {
                continue;
            }

            match *event {
                MovementEvent::TranslateById(id, velocity) => {
                    if is_grounded && entity.index() == id {
                        linear_velocity.x = velocity.x;
                        linear_velocity.z = velocity.y;
                    }
                }
                MovementEvent::RotateById(id, omega) => {
                    if is_grounded && entity.index() == id {
                        angular_velocity.y = omega;
                    }
                }
            }
        }
    }
}
