mod components;
mod events;
mod systems;
mod visual;

use bevy::prelude::*;

pub use crate::interaction_range::components::*;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PickupSet {
    Detect,
    Apply,
}

pub struct InteractionRangePlugin;
impl Plugin for InteractionRangePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<visual::RingAssets>();
        app.add_event::<events::FlagPickupEvent>();
        app.add_event::<events::FlagDropEvent>();
        app.add_event::<events::FlagScoreEvent>();
        app.configure_sets(Update, (PickupSet::Detect, PickupSet::Apply).chain());
        app.add_systems(
            Update,
            (
                systems::update_ring_scale_on_radius_change,
                systems::attach_interaction_range,
                systems::remove_ring_on_radius_removal,
            ),
        );
        app.add_systems(
            Update,
            (systems::detect_flag_pickups, systems::detect_flag_capture).in_set(PickupSet::Detect),
        );
        app.add_systems(
            Update,
            (systems::handle_flag_pickups, systems::handle_flag_capture).in_set(PickupSet::Apply),
        );
    }
}
