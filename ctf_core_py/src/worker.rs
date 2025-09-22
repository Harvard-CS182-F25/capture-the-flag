use crossbeam_channel::{bounded, Receiver, Sender};
use pyo3::prelude::*;

use crate::{agent::parse_actions, game::GameState};
use ctf_core::agent::Action;

pub struct PolicyBridge {
    pub tx_state: Option<Sender<GameState>>,
    pub rx_action: Receiver<Vec<Action>>,
    pub join: Option<std::thread::JoinHandle<()>>,
}

impl PolicyBridge {
    pub fn start_policy_worker(policy: Py<PyAny>) -> Self {
        let (tx_state, rx_state) = bounded::<GameState>(2);
        let (tx_action, rx_action) = bounded::<Vec<Action>>(2);

        let join = std::thread::spawn(move || {
            while let Ok(state) = rx_state.recv() {
                let actions: Vec<Action> = Python::with_gil(|py| -> PyResult<Vec<Action>> {
                    let state_py = Py::new(py, state.clone())?;
                    let actions = parse_actions(
                        py,
                        policy.call_method(py, "get_actions", (state_py,), None)?,
                    );
                    Ok(actions)
                })
                .unwrap_or_else(|e| {
                    eprintln!("Error in policy: {e}");
                    vec![]
                });

                let _ = tx_action.send(actions);
            }
        });

        Self {
            tx_state: Some(tx_state),
            rx_action,
            join: Some(join),
        }
    }

    pub fn shutdown_and_join(&mut self) {
        self.tx_state.take();

        if let Some(j) = self.join.take() {
            let _ = j.join();
        }
    }
}

impl Drop for PolicyBridge {
    fn drop(&mut self) {
        self.shutdown_and_join();
    }
}
