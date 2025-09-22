use avian3d::{math::*, prelude::*};
use bevy::prelude::*;

use super::components::{
    CharacterController, Grounded, MovementAcceleration, MovementDampingFactor,
};
use super::events::MovementEvent;

pub fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let forward = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let backwards = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
    let rotate_left = keyboard_input.any_pressed([KeyCode::KeyQ]);
    let rotate_right = keyboard_input.any_pressed([KeyCode::KeyE]);

    let horizontal = right as i8 - left as i8;
    let vertical = backwards as i8 - forward as i8;
    let rotate = rotate_left as i8 - rotate_right as i8;

    let direction = Vector2::new(horizontal as Scalar, vertical as Scalar).normalize_or_zero();

    if direction != Vector2::ZERO {
        movement_event_writer.write(MovementEvent::Translate(direction));
    }

    if rotate != 0 {
        movement_event_writer.write(MovementEvent::Rotate(rotate as Scalar));
    }
}

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
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementEvent>,
    mut controllers: Query<(
        Entity,
        &MovementAcceleration,
        &mut LinearVelocity,
        &mut AngularVelocity,
        Has<Grounded>,
    )>,
) {
    let delta_time = time.delta_secs();

    for event in movement_event_reader.read() {
        for (
            entity,
            movement_acceleration,
            mut linear_velocity,
            mut angular_velocity,
            is_grounded,
        ) in &mut controllers
        {
            match event {
                MovementEvent::Translate(direction) => {
                    if is_grounded {
                        // rotate the direction vector by the character's rotation
                        // let yaw = rotation.to_euler(EulerRot::YXZ).0;
                        // let rotated_direction = direction.rotate(Vector2::from_angle(yaw));
                        let rotated_direction = direction;
                        linear_velocity.x +=
                            rotated_direction.x * movement_acceleration.0 * delta_time;
                        linear_velocity.z +=
                            rotated_direction.y * movement_acceleration.0 * delta_time;
                    }
                }
                MovementEvent::Rotate(angle) => {
                    if is_grounded {
                        angular_velocity.y += angle * movement_acceleration.0 * delta_time;
                    }
                }
                MovementEvent::TranslateById(id, direction) => {
                    if is_grounded && entity.index() == *id {
                        let rotated_direction = direction;
                        linear_velocity.x +=
                            rotated_direction.x * movement_acceleration.0 * delta_time;
                        linear_velocity.z +=
                            rotated_direction.y * movement_acceleration.0 * delta_time;
                    }
                }
                MovementEvent::RotateById(id, angle) => {
                    if is_grounded && entity.index() == *id {
                        angular_velocity.y += angle * movement_acceleration.0 * delta_time;
                    }
                }
            }
        }
    }
}

pub fn apply_movement_damping(
    mut query: Query<(
        &MovementDampingFactor,
        &mut LinearVelocity,
        &mut AngularVelocity,
    )>,
) {
    for (damping_factor, mut linear_velocity, mut angular_velocity) in &mut query {
        linear_velocity.x *= damping_factor.0;
        linear_velocity.z *= damping_factor.0;
        angular_velocity.y *= damping_factor.0;
    }
}
