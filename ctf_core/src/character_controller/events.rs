use avian3d::math::*;
use bevy::prelude::*;

#[derive(Event)]
pub enum MovementEvent {
    Translate(Vector2),
    Rotate(Scalar),
    TranslateById(u32, Vector2),
    RotateById(u32, Scalar),
}
