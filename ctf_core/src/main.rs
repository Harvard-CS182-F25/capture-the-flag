mod agent;
mod camera;
mod character_controller;
mod core;
mod debug;
mod flag;
mod interaction_range;
mod team;

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Capture the Flag".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            PhysicsPlugins::default(),
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
            debug::DebugPlugin,
            core::CTFPlugin,
        ))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1_500.0,
            ..Default::default()
        })
        .run();
}
