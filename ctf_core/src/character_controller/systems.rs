use avian3d::prelude::*;
use bevy::prelude::*;

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

pub fn movement(
    mut movement_event_reader: EventReader<MovementEvent>,
    mut controllers: Query<(
        Entity,
        &mut LinearVelocity,
        &mut AngularVelocity,
        Has<Grounded>,
    )>,
) {
    for event in movement_event_reader.read() {
        for (entity, mut linear_velocity, mut angular_velocity, is_grounded) in &mut controllers {
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
