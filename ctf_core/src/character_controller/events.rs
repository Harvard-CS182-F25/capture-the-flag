use avian3d::math::*;
use bevy::prelude::*;

#[derive(Event)]
#[allow(dead_code)]
pub enum MovementEvent {
    TranslateById(u32, Vector2),
    RotateById(u32, Scalar),
}
