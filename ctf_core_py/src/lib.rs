mod agent;
mod bridge;
mod game;
mod team;
mod worker;

use avian3d::prelude::*;
use bevy::prelude::*;
use pyo3::prelude::*;

use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use pyo3_stub_gen::define_stub_info_gatherer;

use ctf_core::core;
use ctf_core::debug;

use agent::*;
use game::*;

use crate::team::PyTeamId;

#[pyfunction(name = "run")]
fn run(
    py: Python<'_>,
    red_policy: PyObject,
    blue_policy: PyObject,
    rate_hz: Option<f32>,
) -> PyResult<()> {
    let rate = rate_hz.unwrap_or(60.0).clamp(1.0, 240.0);

    py.allow_threads(|| {
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
            bridge::PythonControlPlugin {
                rate_hz: rate,
                red_policy,
                blue_policy,
            },
        ));

        app.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1_500.0,
            ..Default::default()
        });

        app.run();
    });

    Ok(())
}

#[pymodule]
fn _core(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_class::<AgentState>()?;
    m.add_class::<GameState>()?;
    m.add_class::<PyTeamId>()?;
    m.add_class::<PyAction>()?;

    Ok(())
}

define_stub_info_gatherer!(stub_info);
