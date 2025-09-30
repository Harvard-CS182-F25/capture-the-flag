mod agent;
mod bridge;
mod config;
mod flag;
mod game;
mod state_queue;
mod team;
mod worker;

use avian3d::prelude::*;
use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use ctf_core::team::TeamId;
use pyo3::prelude::*;
use std::time::Duration;

use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use pyo3_stub_gen::define_stub_info_gatherer;

use ctf_core::core;
use ctf_core::debug;

use agent::*;
use game::*;
use pyo3_stub_gen::derive::gen_stub_pyfunction;

use crate::bridge::policy::TestHarnessBridge;
use crate::config::PyConfig;
use crate::flag::CapturePointState;
use crate::flag::FlagState;
use crate::flag::PyFlagStatus;
use crate::state_queue::StateQueue;
use crate::team::PyTeamId;

#[gen_stub_pyfunction]
#[pyfunction(name = "run")]
/// Runs the Capture the Flag simulation with the given policies for each team.
fn run(py: Python<'_>, config: &PyConfig) -> PyResult<()> {
    py.detach(|| {
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
            core::CTFPlugin {
                red_team_agent_positions: config.red_team_agent_positions.clone(),
                blue_team_agent_positions: config.blue_team_agent_positions.clone(),
                red_team_flag_positions: config.red_team_flag_positions.clone(),
                blue_team_flag_positions: config.blue_team_flag_positions.clone(),
                red_team_capture_point_positions: config.red_team_capture_point_positions.clone(),
                blue_team_capture_point_positions: config.blue_team_capture_point_positions.clone(),
                headless: false,
            },
            // Initialize in-proc physics channel + processing system
            bridge::physics::PythonPhysicsBridgePlugin,
        ));

        // Start the physics RPC server now (after physics bridge exists),
        // and export its address so children can connect.
        if let Ok(addr) = bridge::physics::start_physics_rpc_server() {
            std::env::set_var("PHYSICS_ADDR", addr.to_string());
        }

        // Now start policy bridges (children will inherit PHYSICS_ADDR).
        app.add_plugins(bridge::policy::PythonPolicyBridgePlugin {
            config: config.clone(),
            test_harness: None,
        });

        if config.debug {
            app.add_plugins((
                debug::DebugPlugin,
                EguiPlugin::default(),
                WorldInspectorPlugin::new(),
            ));
        }

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
#[pyfunction(name = "run_headless")]
fn run_headless(py: Python<'_>, config: &PyConfig) -> PyResult<StateQueue> {
    let rate = config.rate_hz.unwrap_or(60.0).clamp(1.0, 240.0);
    let config = config.clone();
    let frame_dt = Duration::from_secs_f64(1.0 / rate as f64);

    let (tx_state, rx_state) = crossbeam_channel::bounded::<GameState>(256);
    let (tx_stop, rx_stop) = crossbeam_channel::unbounded::<()>();

    let red_team_agent_positions = config.red_team_agent_positions.clone();
    let blue_team_agent_positions = config.blue_team_agent_positions.clone();
    let red_team_flag_positions = config.red_team_flag_positions.clone();
    let blue_team_flag_positions = config.blue_team_flag_positions.clone();
    let red_team_capture_point_positions = config.red_team_capture_point_positions.clone();
    let blue_team_capture_point_positions = config.blue_team_capture_point_positions.clone();

    let join = py.detach(|| {
        std::thread::spawn(move || {
            let mut app = App::new();

            app.add_plugins((
                DefaultPlugins
                    .build()
                    .disable::<bevy::winit::WinitPlugin>()
                    .disable::<bevy::window::WindowPlugin>()
                    .disable::<bevy::render::RenderPlugin>()
                    .disable::<bevy::pbr::PbrPlugin>()
                    .disable::<bevy::sprite::SpritePlugin>()
                    .disable::<bevy::ui::UiPlugin>()
                    .disable::<bevy::gizmos::GizmoPlugin>()
                    .disable::<PointerInputPlugin>()
                    .disable::<bevy::picking::PickingPlugin>()
                    .disable::<bevy::picking::InteractionPlugin>()
                    .disable::<bevy::text::TextPlugin>()
                    .disable::<bevy::core_pipeline::CorePipelinePlugin>(),
                ScheduleRunnerPlugin::run_loop(frame_dt),
            ));

            // Provide Assets<Mesh> since RenderPlugin is disabled
            app.init_asset::<bevy::render::mesh::Mesh>();

            app.add_plugins((
                PhysicsPlugins::default(),
                core::CTFPlugin {
                    red_team_agent_positions,
                    blue_team_agent_positions,
                    red_team_flag_positions,
                    blue_team_flag_positions,
                    red_team_capture_point_positions,
                    blue_team_capture_point_positions,
                    headless: true,
                },
                // physics bridge first
                bridge::physics::PythonPhysicsBridgePlugin,
            ));

            // Start RPC and export addr so segment_is_free can connect (also
            // useful for same-process tests).
            if let Ok(addr) = bridge::physics::start_physics_rpc_server() {
                std::env::set_var("PHYSICS_ADDR", addr.to_string());
            }

            // Now policy (children inherit PHYSICS_ADDR)
            app.add_plugins(bridge::policy::PythonPolicyBridgePlugin {
                config,
                test_harness: Some(TestHarnessBridge {
                    tx_state: tx_state.clone(),
                    rx_stop: rx_stop.clone(),
                }),
            });

            app.run();
        })
    });

    Ok(StateQueue {
        rx: rx_state,
        tx_stop,
        join: Some(join),
        rate_hz: rate,
    })
}

#[gen_stub_pyfunction]
#[pyfunction(name = "segment_is_free")]
#[pyo3(signature = (start, end, side, timeout_ms=100))]
/// Checks if the line segment from `start` to `end` is free of obstacles by
/// making a blocking RPC to the Bevy app's physics server.
pub fn segment_is_free(
    start: (f32, f32),
    end: (f32, f32),
    side: PyTeamId,
    timeout_ms: u64,
) -> PyResult<bool> {
    use std::io::{BufRead, BufReader, Write};
    use std::net::TcpStream;

    // Get server address from env or from the physics module (same-process).
    let addr = std::env::var("PHYSICS_ADDR")
        .ok()
        .or_else(|| bridge::physics::get_physics_rpc_addr().map(|a| a.to_string()))
        .ok_or_else(|| {
            pyo3::exceptions::PyRuntimeError::new_err(
                "Physics RPC not initialized (PHYSICS_ADDR not set)",
            )
        })?;

    let stream = TcpStream::connect(&addr).map_err(|e| {
        pyo3::exceptions::PyConnectionError::new_err(format!("connect {}: {}", addr, e))
    })?;
    let _ = stream.set_read_timeout(Some(Duration::from_millis(timeout_ms)));
    let _ = stream.set_write_timeout(Some(Duration::from_millis(50)));

    // Simple one-shot request with id=1.
    let team_str = match side.inner {
        TeamId::Red => "Red",
        TeamId::Blue => "Blue",
    };
    let req = serde_json::json!({
        "id": 1u64,
        "start": [start.0, start.1],
        "end":   [end.0,   end.1],
        "team":  team_str,
    });

    {
        let mut w = std::io::BufWriter::new(&stream);
        writeln!(w, "{}", req).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("send physics req: {}", e))
        })?;
        let _ = w.flush();
    }

    let mut r = BufReader::new(stream);
    let mut line = String::new();
    r.read_line(&mut line).map_err(|_| {
        pyo3::exceptions::PyTimeoutError::new_err("Timeout waiting for physics response")
    })?;

    #[derive(serde::Deserialize)]
    struct PhysResp {
        id: u64,
        free: bool,
    }
    let resp: PhysResp = serde_json::from_str(&line).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("bad physics resp: {}", e))
    })?;

    Ok(resp.free)
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

#[pymodule(gil_used = false)]
fn _core(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(run_headless, m)?)?;
    m.add_function(wrap_pyfunction!(segment_is_free, m)?)?;
    m.add_class::<AgentState>()?;
    m.add_class::<GameState>()?;
    m.add_class::<FlagState>()?;
    m.add_class::<CapturePointState>()?;
    m.add_class::<PyConfig>()?;
    m.add_class::<PyFlagStatus>()?;
    m.add_class::<PyTeamId>()?;
    m.add_class::<PyAction>()?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
