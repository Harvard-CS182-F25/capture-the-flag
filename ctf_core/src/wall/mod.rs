mod components;
mod systems;
mod visual;

use bevy::prelude::*;

pub use components::*;
pub use visual::*;

use crate::core::CTFConfig;

pub const COLLISION_LAYER_WALL: u32 = 1 << 0;
pub const WALL_HEIGHT: f32 = 5.0;
pub const WALL_THICKNESS: f32 = 1.0;

pub struct WallPlugin;
impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreStartup,
            init_wall_assets.run_if(|c: Res<CTFConfig>| !c.headless),
        );
        app.add_systems(
            Startup,
            systems::spawn_walls.run_if(|c: Res<CTFConfig>| !c.headless),
        );
        app.add_systems(
            Startup,
            systems::spawn_walls_headless.run_if(|c: Res<CTFConfig>| c.headless),
        );
    }
}

fn init_wall_assets(mut commands: Commands) {
    commands.init_resource::<WallGraphicsAssets>();
}
