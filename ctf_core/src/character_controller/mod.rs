use bevy::prelude::*;

mod components;
mod events;
mod systems;

pub use components::*;

pub struct CharacterControllerPlugin;
impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::MovementEvent>().add_systems(
            Update,
            (
                systems::keyboard_input,
                systems::update_grounded,
                systems::movement,
                systems::apply_movement_damping,
            ),
        );
    }
}
