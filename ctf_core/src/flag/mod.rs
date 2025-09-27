mod components;
mod systems;
mod visual;

use bevy::prelude::*;

pub use components::*;

pub struct FlagPlugin;
impl Plugin for FlagPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<components::FlagCaptureCounts>();
        app.init_resource::<visual::FlagGraphicsAssets>();
        app.init_resource::<visual::CapturePointGraphicsAssets>();
        app.init_resource::<components::FlagCaptureCounts>();
        app.add_systems(Startup, systems::spawn_flags);
        app.add_systems(Startup, systems::spawn_capture_points);
    }
}
