use bevy::prelude::*;

#[derive(Event, Clone, Copy)]
pub struct FlagPickupEvent {
    pub agent: Entity,
    pub flag: Entity,
}

#[derive(Event, Clone, Copy)]
pub struct FlagDropEvent {
    pub agent: Entity,
    pub flag: Entity,
}

#[derive(Event, Clone, Copy)]
pub struct FlagScoreEvent {
    pub agent: Entity,
    pub flag: Entity,
}
