use bevy::{math::NormedVectorSpace, prelude::*};
use crossbeam_channel::{Receiver, Sender, TrySendError};
use ctf_core::{
    agent::{Action, Agent},
    character_controller::MovementEvent,
    flag::{CapturePoint, Flag, FlagCaptureCounts},
    team::{Team, TeamId},
};

use crate::{
    agent::collect_agent_states,
    config::PyConfig,
    flag::{collect_capture_point_states, collect_flag_states},
    game::GameState,
    worker::policy::PolicyBridge,
};

#[derive(Resource)]
struct Bridge {
    red: PolicyBridge,
    blue: PolicyBridge,
    test: Option<TestHarnessBridge>,
}

#[derive(Clone)]
pub struct TestHarnessBridge {
    pub tx_state: Sender<GameState>,
    pub rx_stop: Receiver<()>,
}

#[derive(Resource)]
struct PolicyTimer(Timer);

pub struct PythonPolicyBridgePlugin {
    pub config: PyConfig,
    pub test_harness: Option<TestHarnessBridge>,
}

impl Plugin for PythonPolicyBridgePlugin {
    fn build(&self, app: &mut App) {
        let hz = self.config.rate_hz.unwrap_or(60.0).clamp(1.0, 240.0);
        let interval = 1.0_f32 / hz;

        let red_bridge =
            PolicyBridge::start(TeamId::Red, self.config.clone(), &self.config.python_exe)
                .expect("Failed to start red policy");
        let blue_bridge =
            PolicyBridge::start(TeamId::Blue, self.config.clone(), &self.config.python_exe)
                .expect("Failed to start red policy");

        app.insert_resource(PolicyTimer(Timer::from_seconds(
            interval,
            TimerMode::Repeating,
        )));

        app.insert_resource(Bridge {
            red: red_bridge,
            blue: blue_bridge,
            test: self.test_harness.clone(),
        });

        app.add_systems(
            Update,
            (send_game_states, apply_actions, on_test_harness_stop),
        );

        app.add_systems(Last, shutdown_workers_on_exit);
    }
}

fn on_test_harness_stop(bridge: Option<Res<Bridge>>, mut exit: EventWriter<AppExit>) {
    let Some(bridge) = bridge else {
        return;
    };
    if let Some(test) = &bridge.test {
        if test.rx_stop.try_recv().is_ok() {
            println!("Test harness requested stop; exiting");

            exit.write(AppExit::Success);
        }
    }
}

fn send_game_states(
    time: Res<Time>,
    mut t: ResMut<PolicyTimer>,
    scores: Res<FlagCaptureCounts>,
    bridge: Option<Res<Bridge>>,
    agents: Query<(Entity, &Name, &Transform, &Agent, &Team)>,
    flags: Query<(Entity, &Name, &Transform, &Flag)>,
    capture_points: Query<(Entity, &Name, &Transform, &CapturePoint)>,
) {
    let Some(bridge) = bridge else {
        return;
    };

    if !t.0.tick(time.delta()).just_finished() {
        return;
    }

    let (red_team, blue_team) = collect_agent_states(agents);
    let (red_flags, blue_flags) = collect_flag_states(flags);
    let (red_capture_points, blue_capture_points) = collect_capture_point_states(capture_points);
    let num_flags_per_team = red_flags.len() as u32;

    let game_state = GameState {
        red_score: scores.red,
        blue_score: scores.blue,
        red_team,
        blue_team,
        red_flags,
        blue_flags,
        num_flags_per_team,
        red_capture_points,
        blue_capture_points,
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

    if let Some(test) = &bridge.test {
        match test.tx_state.try_send(game_state) {
            Ok(_) => {}
            Err(TrySendError::Full(_)) => {}
            Err(TrySendError::Disconnected(_)) => { /* worker died; you may log */ }
        }
    }
}

fn apply_actions(
    bridge: Option<Res<Bridge>>,
    agents: Query<(Entity, &Agent)>,
    mut movement_event_writer: EventWriter<MovementEvent>,
) {
    let Some(bridge) = bridge else {
        return;
    };

    for bridge in [&bridge.red, &bridge.blue] {
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
                    velocity,
                } => {
                    let agent = agents.iter().find(|(e, _a)| e.index() == agent_id);
                    if agent.is_none() {
                        eprintln!("No agent with id {agent_id}");
                        continue;
                    }
                    let (_, agent) = agent.unwrap();
                    let velocity = if velocity.norm() > agent.speed {
                        eprintln!(
                            "Agent {agent_id} trying to move too fast: {} > {}",
                            velocity.norm(),
                            agent.speed
                        );
                        eprintln!("Capping to max speed");
                        velocity.normalize() * agent.speed
                    } else {
                        velocity
                    };
                    movement_event_writer.write(MovementEvent::TranslateById(agent_id, velocity));
                }
            }
        }
    }
}

fn shutdown_workers_on_exit(mut exit_ev: EventReader<AppExit>, mut bridge: Option<ResMut<Bridge>>) {
    if exit_ev.read().next().is_none() {
        return;
    }

    bridge.take();
}
