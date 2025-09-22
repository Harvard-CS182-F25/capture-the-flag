mod components;
mod systems;
mod visual;

use bevy::prelude::*;

pub use crate::flag::components::*;

pub struct FlagPlugin;
impl Plugin for FlagPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<visual::FlagGraphicsAssets>();
        app.init_resource::<visual::CapturePointGraphicsAssets>();
        app.add_systems(Startup, systems::spawn_flags);
    }
}
