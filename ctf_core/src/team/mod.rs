use bevy::prelude::*;

pub use crate::team::components::*;

mod components;

pub const COLLISION_LAYER_RED: u32 = 1 << 2;
pub const COLLISION_LAYER_BLUE: u32 = 1 << 3;

pub struct TeamPlugin;
impl Plugin for TeamPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TeamId>();
    }
}
