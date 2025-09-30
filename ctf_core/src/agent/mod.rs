mod components;
mod systems;
mod visual;

use bevy::prelude::*;

pub use components::*;
pub use visual::*;

use crate::core::CTFConfig;

pub const COLLISION_LAYER_AGENT: u32 = 1 << 1;

pub struct AgentPlugin;
impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Agent>();
        app.add_systems(
            PreStartup,
            spawn_agent_assets.run_if(|c: Res<CTFConfig>| !c.headless),
        );
        app.add_systems(
            Startup,
            systems::spawn_agents.run_if(|c: Res<CTFConfig>| !c.headless),
        );
        app.add_systems(
            Startup,
            systems::spawn_agents_headless.run_if(|c: Res<CTFConfig>| c.headless),
        );
    }
}

fn spawn_agent_assets(mut commands: Commands) {
    commands.init_resource::<visual::AgentGraphicsAssets>();
}
