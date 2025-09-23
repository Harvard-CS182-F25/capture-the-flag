use bevy::prelude::*;

mod components;
mod events;
mod systems;

pub use components::*;
#[allow(unused_imports)]
pub use events::*;

pub struct CharacterControllerPlugin;
impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::MovementEvent>()
            .add_systems(Update, (systems::update_grounded, systems::movement));
    }
}
