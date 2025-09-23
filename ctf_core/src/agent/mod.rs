mod components;
mod systems;
mod visual;

use bevy::prelude::*;

pub use components::*;
pub use visual::*;

pub const COLLISION_LAYER_AGENT: u32 = 1 << 1;

pub struct AgentPlugin;
impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Agent>();
        app.init_resource::<visual::AgentGraphicsAssets>();
        app.add_systems(Startup, systems::spawn_agents);
    }
}
