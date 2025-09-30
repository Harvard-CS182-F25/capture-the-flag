use std::sync::RwLock;

use avian3d::{math::Quaternion, prelude::*};
use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};
use ctf_core::flag::{COLLISION_LAYER_CAMP_BLOCK_BLUE, COLLISION_LAYER_CAMP_BLOCK_RED};
use ctf_core::team::TeamId;
use ctf_core::wall::COLLISION_LAYER_WALL;
use once_cell::sync::Lazy;

// --- public query types used in-proc ---
#[derive(Debug, Clone, Copy)]
pub struct Segment2D {
    pub start: Vec2, // (x,z) on ground plane
    pub end: Vec2,
}

pub enum PhysicsQuery {
    SegmentCollision2D {
        seg: Segment2D,
        team_id: TeamId,
        reply: Sender<bool>, // one-shot back to caller
    },
}

#[derive(Resource)]
pub struct PhysicsRx(pub Receiver<PhysicsQuery>);

// Replaceable global sender (set by the plugin)
pub static PHYSICS_TX: Lazy<RwLock<Option<Sender<PhysicsQuery>>>> = Lazy::new(|| RwLock::new(None));

pub fn set_physics_tx(tx: Sender<PhysicsQuery>) {
    *PHYSICS_TX.write().unwrap() = Some(tx);
}
pub fn get_physics_tx() -> Option<Sender<PhysicsQuery>> {
    PHYSICS_TX.read().unwrap().clone()
}

// --- Bevy plugin to init the in-proc channel and system ---
pub struct PythonPhysicsBridgePlugin;
impl Plugin for PythonPhysicsBridgePlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = crossbeam_channel::unbounded::<PhysicsQuery>();
        set_physics_tx(tx);
        app.insert_resource(PhysicsRx(rx));
        app.add_systems(Update, process_physics_queries);
    }
}

// System that consumes PhysicsQuery and performs the casts.
pub fn process_physics_queries(receiver: Res<PhysicsRx>, spatial: SpatialQuery) {
    for q in receiver.0.try_iter() {
        match q {
            PhysicsQuery::SegmentCollision2D {
                seg,
                team_id,
                reply,
            } => {
                let collided = segment_hits_wall_flag_or_capture_point(&spatial, seg, team_id);
                let _ = reply.send(collided);
            }
        }
    }
}

// Core collision helper used by both in-proc and RPC paths.
fn segment_hits_wall_flag_or_capture_point(
    spatial: &SpatialQuery,
    seg: Segment2D,
    team_id: TeamId,
) -> bool {
    let shape = Collider::cuboid(1.0, 1.0, 1.0);

    // 1) Walls: simple mask
    let block_mask = match team_id {
        TeamId::Red => COLLISION_LAYER_WALL | COLLISION_LAYER_CAMP_BLOCK_RED,
        TeamId::Blue => COLLISION_LAYER_WALL | COLLISION_LAYER_CAMP_BLOCK_BLUE,
    };
    let filter = SpatialQueryFilter::from_mask(LayerMask(block_mask));

    let start = Vec3::new(seg.start.x, 0.5, seg.start.y);
    let end = Vec3::new(seg.end.x, 0.5, seg.end.y);
    let delta = end - start;
    let dist = delta.length();
    let rot = Quaternion::IDENTITY;

    if dist <= f32::EPSILON
        && !spatial
            .shape_intersections(&shape, start, rot, &filter)
            .is_empty()
    {
        return true;
    }

    spatial
        .cast_shape(
            &shape,
            start,
            rot,
            Dir3::new(delta).unwrap(),
            &ShapeCastConfig::from_max_distance(dist),
            &filter,
        )
        .is_some()
}

// --- Tiny TCP RPC server so other processes can ask for segment_is_free ---

use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    thread,
    time::Duration,
};

static PHYSICS_RPC_ADDR: Lazy<RwLock<Option<SocketAddr>>> = Lazy::new(|| RwLock::new(None));

pub fn get_physics_rpc_addr() -> Option<SocketAddr> {
    *PHYSICS_RPC_ADDR.read().unwrap()
}
fn set_physics_rpc_addr(addr: SocketAddr) {
    *PHYSICS_RPC_ADDR.write().unwrap() = Some(addr);
}

#[derive(Deserialize)]
struct PhysReq {
    id: u64,
    start: [f32; 2],
    end: [f32; 2],
    team: String, // "Red" | "Blue"
}
#[derive(Serialize)]
struct PhysResp {
    id: u64,
    free: bool,
}

/// Start the line-based TCP physics RPC server on 127.0.0.1:0.
/// Returns the bound address.
pub fn start_physics_rpc_server() -> std::io::Result<SocketAddr> {
    let physics_tx =
        get_physics_tx().expect("physics channel must be initialized before starting RPC");
    let listener = TcpListener::bind(("127.0.0.1", 0))?;
    let addr = listener.local_addr()?;
    set_physics_rpc_addr(addr);

    thread::spawn(move || {
        for conn in listener.incoming() {
            match conn {
                Ok(stream) => {
                    let tx = physics_tx.clone();
                    thread::spawn(move || handle_conn(stream, tx));
                }
                Err(_e) => break,
            }
        }
    });

    Ok(addr)
}

fn handle_conn(s: TcpStream, physics_tx: Sender<PhysicsQuery>) {
    let _ = s.set_read_timeout(Some(Duration::from_millis(500)));
    let _ = s.set_write_timeout(Some(Duration::from_millis(500)));

    let mut r = BufReader::new(s.try_clone().unwrap());
    let mut w = std::io::BufWriter::new(s);
    let mut line = String::new();

    loop {
        line.clear();
        let n = match r.read_line(&mut line) {
            Ok(n) => n,
            Err(_) => break,
        };
        if n == 0 {
            break;
        }

        let req: PhysReq = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let (tx, rx) = crossbeam_channel::bounded(1);
        let team_id = if req.team.eq_ignore_ascii_case("red") {
            TeamId::Red
        } else {
            TeamId::Blue
        };

        let _ = physics_tx.send(PhysicsQuery::SegmentCollision2D {
            seg: Segment2D {
                start: Vec2::new(req.start[0], req.start[1]),
                end: Vec2::new(req.end[0], req.end[1]),
            },
            team_id,
            reply: tx,
        });

        let collided = rx.recv_timeout(Duration::from_millis(250)).unwrap_or(true);
        let _ = writeln!(
            w,
            "{}",
            serde_json::to_string(&PhysResp {
                id: req.id,
                free: !collided
            })
            .unwrap()
        );
        let _ = w.flush();
    }
}
