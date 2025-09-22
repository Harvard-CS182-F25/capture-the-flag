use avian3d::prelude::*;
use bevy::prelude::*;
use pyo3::prelude::*;

use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use pyo3_stub_gen::define_stub_info_gatherer;

use ctf_core::core;
use ctf_core::debug;

#[pyfunction(name = "run")]
fn run(_py: Python<'_>) -> PyResult<()> {
    let mut app = App::new();
    app.add_plugins((
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
    ));

    app.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1_500.0,
        ..Default::default()
    });

    app.run();
    Ok(())
}

#[pymodule]
fn _core(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
