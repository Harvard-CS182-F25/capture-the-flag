use bevy::math::Vec2;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

use crate::{
    agent::{AgentState, PyAction},
    game::GameState,
    team::PyTeamId,
};

#[gen_stub_pyclass]
#[pyclass]
pub struct DefenseBot {
    side: PyTeamId,
}

#[gen_stub_pymethods]
#[pymethods]
impl DefenseBot {
    #[new]
    fn new(side: PyTeamId) -> Self {
        DefenseBot { side }
    }

    #[allow(unused_variables)]
    fn startup(&self, initial_state: GameState) {}

    fn get_action(&self, game_state: GameState, agent_state: AgentState) -> PyAction {
        // Basic params
        let r_target: f32 = 5.0;
        let k_p: f32 = 2.0; // position gain toward target point
        let k_rad: f32 = 1.5; // radial correction gain
        let tangential_speed_frac: f32 = 0.35; // fraction of max speed for orbit

        let self_pos = Vec2::from(agent_state.position);
        let max_speed = agent_state.max_speed();

        // --- find closest own flag (defense anchor) ---
        let own_flags = game_state.get_team_flags(&self.side);
        let closest_flag = own_flags.iter().min_by(|a, b| {
            let da = (Vec2::from(a.position) - self_pos).length_squared();
            let db = (Vec2::from(b.position) - self_pos).length_squared();
            da.partial_cmp(&db).unwrap()
        });

        // --- opponents + potential carrier ---
        let opponents = game_state.get_team_agents(&self.side.other());
        let closest_opp = opponents.iter().min_by(|a, b| {
            let da = (Vec2::from(a.position) - self_pos).length_squared();
            let db = (Vec2::from(b.position) - self_pos).length_squared();
            da.partial_cmp(&db).unwrap()
        });

        // Heuristic: if any opponent is carrying *a* flag, treat them as carrier to chase.
        // (In CTF, opponents can only carry our flag, so this is usually correct.)
        let carrier = opponents.iter().find(|a| a.has_flag());

        let desired_vel: Vec2 = if let Some(carrier) = carrier {
            // --- CHASE: go straight at the flag carrier ---
            let target = Vec2::from(carrier.position);
            steer_toward(self_pos, target, max_speed)
        } else if let Some(flag) = closest_flag {
            // --- DEFENSE: hold a ring around the flag, aligned to closest opponent ---
            let flag_pos = Vec2::from(flag.position);

            // radial vector from flag to us
            let r = self_pos - flag_pos;
            let r_len = r.length();
            let r_dir = if r_len > 1e-4 { r / r_len } else { Vec2::X };

            // direction from flag to closest opponent (if none, use current radial dir)
            let opp_dir = closest_opp
                .map(|o| {
                    let d = Vec2::from(o.position) - flag_pos;
                    let dl = d.length();
                    if dl > 1e-4 {
                        d / dl
                    } else {
                        r_dir
                    }
                })
                .unwrap_or(r_dir);

            // target point on the circle of radius r_target toward opponent
            let circle_target = flag_pos + opp_dir * r_target;

            // proportional move toward that target point
            let p_term = (circle_target - self_pos) * k_p;

            // radial correction to stay near r_target
            let radial_error = r_len - r_target;
            let rad_term = -r_dir * (radial_error * k_rad);

            // tangential (perpendicular) to keep orbiting
            let tang_dir = Vec2::new(-r_dir.y, r_dir.x); // 90° CCW
            let tang_term = tang_dir * (tangential_speed_frac * max_speed);

            let v = p_term + rad_term + tang_term;
            clamp_len(v, max_speed - f32::EPSILON)
        } else {
            // No flags? idle.
            Vec2::ZERO
        };

        PyAction::new(agent_state.id, (desired_vel.x, desired_vel.y))
    }
}

// --- helpers ---
fn clamp_len(v: Vec2, max: f32) -> Vec2 {
    let len = v.length();
    if len > max && max > 0.0 {
        v * (max / len)
    } else {
        v
    }
}

fn steer_toward(from: Vec2, to: Vec2, max_speed: f32) -> Vec2 {
    let d = to - from;
    let l = d.length();
    if l > 1e-4 {
        d * (max_speed / l)
    } else {
        Vec2::ZERO
    }
}
