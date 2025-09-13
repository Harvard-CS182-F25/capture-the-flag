use avian3d::math::*;
use bevy::prelude::*;

#[derive(Event)]
pub enum MovementEvent {
    Translate(Vector2),
    Rotate(Scalar),
}
