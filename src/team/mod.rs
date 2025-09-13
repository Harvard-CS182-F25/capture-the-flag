use bevy::prelude::*;

pub use crate::team::components::*;

mod components;

pub struct TeamPlugin;
impl Plugin for TeamPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TeamId>();
    }
}
