mod components;
mod systems;
mod visual;

use bevy::prelude::*;

pub use components::*;
pub use visual::*;

pub struct AgentPlugin;
impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Agent>();
        app.init_resource::<visual::AgentGraphicsAssets>();
        app.add_systems(Startup, systems::spawn_agents);
    }
}
