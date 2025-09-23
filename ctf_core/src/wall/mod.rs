mod components;
mod systems;
mod visual;

use bevy::prelude::*;

pub use components::*;
pub use visual::*;

pub const COLLISION_LAYER_WALL: u32 = 1 << 0;
pub struct WallPlugin;
impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<visual::WallGraphicsAssets>();
        app.add_systems(Startup, systems::spawn_walls);
    }
}
