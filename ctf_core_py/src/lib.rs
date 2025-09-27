mod agent;
mod bridge;
mod flag;
mod game;
mod team;
mod worker;

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use pyo3::prelude::*;

use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use pyo3_stub_gen::define_stub_info_gatherer;

use ctf_core::core;
use ctf_core::debug;

use agent::*;
use game::*;
use pyo3_stub_gen::derive::gen_stub_pyfunction;

use crate::flag::FlagState;
use crate::flag::PyFlagStatus;
use crate::team::PyTeamId;

#[gen_stub_pyfunction]
#[pyfunction(name = "run")]
/// Runs the Capture the Flag simulation with the given policies for each team.
///
/// Parameters
///     `start`: A tuple (x, y) representing the start point of the segment.
///     `end`: A tuple (x, y) representing the end point of the segment.
///     `timeout_ms`: Optional timeout in milliseconds to wait for a response from the physics engine. Default is 100ms.
///
/// Returns
///    `True` if the agent can move along the segment without colliding with any obstacles
///    `False` otherwise
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
            bridge::policy::PythonPolicyBridgePlugin {
                rate_hz: rate,
                red_policy,
                blue_policy,
            },
            bridge::physics::PythonPhysicsBridgePlugin,
        ));

        app.add_systems(PostStartup, force_focus);

        app.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1_500.0,
            ..Default::default()
        });

        app.run();
    });

    Ok(())
}

#[gen_stub_pyfunction]
#[pyfunction(name = "segment_is_free")]
#[pyo3(signature = (start, end, timeout_ms=100))]
/// Checks if the line segment from `start` to `end` is free of obstacles. The shape of agent is swept along
/// this segment to check for collisions.
///
/// Parameters
///     `start`: A tuple (x, y) representing the start point of the segment.
///     `end`: A tuple (x, y) representing the end point of the segment.
///     `timeout_ms`: Optional timeout in milliseconds to wait for a response from the physics engine. Default is 100ms.
///
/// Returns
///    `True` if the agent can move along the segment without colliding with any obstacles
///    `False` otherwise
pub fn segment_is_free(start: (f32, f32), end: (f32, f32), timeout_ms: u64) -> PyResult<bool> {
    let start = Vec2::new(start.0, start.1);
    let end = Vec2::new(end.0, end.1);

    let (tx, rx) = crossbeam_channel::bounded::<bool>(1);

    if let Some(physics_tx) = bridge::physics::PHYSICS_TX.get() {
        let query = bridge::physics::PhysicsQuery::SegmentCollision2D {
            seg: bridge::physics::Segment2D { start, end },
            reply: tx,
        };
        if physics_tx.send(query).is_err() {
            return Err(pyo3::exceptions::PyRuntimeError::new_err(
                "Failed to send physics query",
            ));
        }
    } else {
        return Err(pyo3::exceptions::PyRuntimeError::new_err(
            "Physics bridge not initialized",
        ));
    }

    let ok = Python::with_gil(|py| {
        py.allow_threads(|| rx.recv_timeout(std::time::Duration::from_millis(timeout_ms)))
    });

    match ok {
        Ok(collided) => Ok(!collided), // if collided, not free
        Err(_) => Err(pyo3::exceptions::PyTimeoutError::new_err(
            "Timeout waiting for physics response",
        )),
    }
}

fn force_focus(
    winit_windows: NonSend<WinitWindows>,
    q: Query<(Entity, &Window), With<PrimaryWindow>>,
) {
    if let Ok((entity, _window)) = q.single() {
        if let Some(win) = winit_windows.get_window(entity) {
            // winit 0.29+: request focus (no-op on some platforms)
            win.focus_window();
        }
    }
}

#[pymodule]
fn _core(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(segment_is_free, m)?)?;
    m.add_class::<AgentState>()?;
    m.add_class::<GameState>()?;
    m.add_class::<FlagState>()?;
    m.add_class::<PyFlagStatus>()?;
    m.add_class::<PyTeamId>()?;
    m.add_class::<PyAction>()?;

    Ok(())
}

define_stub_info_gatherer!(stub_info);
