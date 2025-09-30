use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
};

use crossbeam_channel::{bounded, Receiver, Sender, TrySendError};

use crate::{config::PyConfig, game::GameState, team::PyTeamId};
use ctf_core::{agent::Action, team::TeamId};

pub struct PolicyBridge {
    pub tx_state: Option<Sender<GameState>>,
    pub rx_action: Receiver<Vec<Action>>,
}

fn run_io_loop(
    mut child: Child,
    mut stdin: impl std::io::Write + Send + 'static,
    stdout: impl std::io::Read + Send + 'static,
    rx_state: Receiver<GameState>,
    tx_action: Sender<Vec<Action>>,
) {
    let (tx_err, rx_err) = bounded::<()>(1);

    let tx_action_clone = tx_action.clone();
    let reader = std::thread::spawn(move || {
        let mut buf_reader = BufReader::new(stdout);
        let mut line = String::new();
        loop {
            line.clear();
            match buf_reader.read_line(&mut line) {
                Ok(0) => break, // EOF
                Ok(_) => match serde_json::from_str::<Vec<Action>>(&line) {
                    Ok(actions) => {
                        match tx_action_clone.try_send(actions) {
                            Ok(()) => {}
                            Err(TrySendError::Full(_)) => {
                                // drop newest (or choose a strategy to prefer latest)
                                // simplest: just drop; apply_actions will consume older pending ones this frame
                            }
                            Err(TrySendError::Disconnected(_)) => break,
                        };
                    }
                    Err(e) => eprintln!("Failed to parse actions from policy: {}, {}", line, e),
                },
                Err(_) => break,
            }
        }
        let _ = tx_err.send(());
    });

    while let Ok(state) = rx_state.recv() {
        if serde_json::to_writer(&mut stdin, &state).is_ok() {
            let _ = stdin.write_all(b"\n");
            let _ = stdin.flush();
        } else {
            break;
        }

        if rx_err.try_recv().is_ok() {
            break;
        }
    }

    let _ = child.kill();
    let _ = reader.join();
}

impl PolicyBridge {
    pub fn start(side: TeamId, config: PyConfig, python_exe: &str) -> anyhow::Result<Self> {
        let config_json = serde_json::to_string(&config)?;
        let side_json = serde_json::to_string(&PyTeamId { inner: side })?;

        let mut child = Command::new(python_exe)
            .args(["run", "launch-team", &config_json, "--side", &side_json])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();

        let (tx_state, rx_state) = bounded::<GameState>(2);
        let (tx_action, rx_action) = bounded::<Vec<Action>>(2);

        std::thread::spawn(move || run_io_loop(child, stdin, stdout, rx_state, tx_action));

        Ok(Self {
            tx_state: Some(tx_state),
            rx_action,
        })
    }

    pub fn shutdown_and_join(&mut self) {
        self.tx_state.take();
    }
}

impl Drop for PolicyBridge {
    fn drop(&mut self) {
        self.shutdown_and_join();
    }
}
