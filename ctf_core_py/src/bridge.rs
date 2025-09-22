use bevy::prelude::*;
use crossbeam_channel::TrySendError;
use ctf_core::{
    agent::{Action, Agent},
    character_controller::MovementEvent,
    team::Team,
};

use crate::{agent::collect_agent_states, game::GameState, worker::PolicyBridge};
use pyo3::prelude::*;

#[derive(Resource)]
struct Bridge {
    red: PolicyBridge,
    blue: PolicyBridge,
}

#[derive(Resource)]
struct PolicyTimer(Timer);

pub struct PythonControlPlugin {
    pub rate_hz: f32,
    pub red_policy: PyObject,
    pub blue_policy: PyObject,
}

impl Plugin for PythonControlPlugin {
    fn build(&self, app: &mut App) {
        let interval = 1.0_f32 / self.rate_hz.max(1.0);
        let red_bridge =
            Python::with_gil(|py| PolicyBridge::start_policy_worker(self.red_policy.clone_ref(py)));
        let blue_bridge = Python::with_gil(|py| {
            PolicyBridge::start_policy_worker(self.blue_policy.clone_ref(py))
        });

        app.insert_resource(PolicyTimer(Timer::from_seconds(
            interval,
            TimerMode::Repeating,
        )));

        app.insert_resource(Bridge {
            red: red_bridge,
            blue: blue_bridge,
        });

        app.add_systems(Update, (send_game_states, apply_actions));
        app.add_systems(Last, join_workers_on_exit);
    }
}

fn send_game_states(
    time: Res<Time>,
    mut t: ResMut<PolicyTimer>,
    bridge: Option<Res<Bridge>>,
    agents: Query<(Entity, &Name, &Transform, &Agent, &Team)>,
) {
    let Some(bridge) = bridge else {
        return;
    };

    if !t.0.tick(time.delta()).just_finished() {
        return;
    }

    let (red_team, blue_team) = collect_agent_states(agents);
    let game_state = GameState {
        red_score: 0,
        blue_score: 0,
        red_team,
        blue_team,
    };

    for bridge in [&bridge.red, &bridge.blue] {
        match bridge
            .tx_state
            .as_ref()
            .unwrap()
            .try_send(game_state.clone())
        {
            Ok(_) => {}
            Err(TrySendError::Full(_)) => { /* worker still busy; skip this one */ }
            Err(TrySendError::Disconnected(_)) => { /* worker died; you may log */ }
        }
    }
}

fn apply_actions(
    bridge: Option<Res<Bridge>>,
    mut movement_event_writer: EventWriter<MovementEvent>,
) {
    let Some(bridge) = bridge else {
        return;
    };

    for bridge in [&bridge.red, &bridge.blue] {
        // Drain to latest
        let mut latest: Option<Vec<Action>> = None;
        while let Ok(a) = bridge.rx_action.try_recv() {
            latest = Some(a);
        }
        let Some(actions) = latest else {
            continue;
        };

        for act in actions {
            match act {
                Action::Move {
                    id: agent_id,
                    direction,
                } => {
                    movement_event_writer.write(MovementEvent::TranslateById(agent_id, direction));
                }
            }
        }
    }
}

fn join_workers_on_exit(mut exit_ev: EventReader<AppExit>, bridge: Option<ResMut<Bridge>>) {
    // Run once when we see the first exit event
    if exit_ev.read().next().is_none() {
        return;
    }
    if let Some(mut b) = bridge {
        b.red.shutdown_and_join();
        b.blue.shutdown_and_join();
    }
}
