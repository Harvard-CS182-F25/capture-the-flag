mod components;
mod systems;
mod visual;

use bevy::prelude::*;

pub use components::*;

use crate::core::CTFConfig;

pub const FLAG_COOLDOWN_TIME: f32 = 0.5;
pub const FLAG_INTERACTION_RADIUS: f32 = 3.0;
pub const FLAG_SPAWN_RADIUS: f32 = 5.0;
pub const KEEP_AWAY_RADIUS: f32 = 3.0;

pub const COLLISION_LAYER_FLAG_OR_CP: u32 = 1 << 5;
pub const COLLISION_LAYER_CAMP_BLOCK_RED: u32 = 1 << 20;
pub const COLLISION_LAYER_CAMP_BLOCK_BLUE: u32 = 1 << 21;

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
