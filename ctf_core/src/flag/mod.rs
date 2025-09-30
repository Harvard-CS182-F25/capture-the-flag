mod components;
mod systems;
mod visual;

use bevy::prelude::*;

pub use components::*;

use crate::core::CTFConfig;

pub struct FlagPlugin;
impl Plugin for FlagPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<components::FlagCaptureCounts>();
        app.init_resource::<components::FlagCaptureCounts>();
        app.add_systems(
            PreStartup,
            init_flag_and_capture_point_assets.run_if(|c: Res<CTFConfig>| !c.headless),
        );
        app.add_systems(
            Startup,
            systems::spawn_flags.run_if(|c: Res<CTFConfig>| !c.headless),
        );
        app.add_systems(
            Startup,
            systems::spawn_flags_headless.run_if(|c: Res<CTFConfig>| c.headless),
        );
        app.add_systems(
            Startup,
            systems::spawn_capture_points.run_if(|c: Res<CTFConfig>| !c.headless),
        );
        app.add_systems(
            Startup,
            systems::spawn_capture_points_headless.run_if(|c: Res<CTFConfig>| c.headless),
        );
    }
}

fn init_flag_and_capture_point_assets(mut commands: Commands) {
    commands.init_resource::<visual::FlagGraphicsAssets>();
    commands.init_resource::<visual::CapturePointGraphicsAssets>();
}
