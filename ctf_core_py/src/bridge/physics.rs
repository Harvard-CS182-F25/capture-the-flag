use avian3d::{math::Quaternion, prelude::*};
use bevy::prelude::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use ctf_core::wall::COLLISION_LAYER_WALL;

#[derive(Debug, Clone, Copy)]
pub struct Segment2D {
    pub start: Vec2, // (x,z) on ground plane
    pub end: Vec2,
}

pub enum PhysicsQuery {
    SegmentCollision2D {
        seg: Segment2D,
        reply: Sender<bool>, // one-shot back to caller
    },
}

#[derive(Resource)]
pub struct PhysicsRx(pub Receiver<PhysicsQuery>);

pub static PHYSICS_TX: once_cell::sync::OnceCell<Sender<PhysicsQuery>> =
    once_cell::sync::OnceCell::new();

pub struct PythonPhysicsBridgePlugin;
impl Plugin for PythonPhysicsBridgePlugin {
    fn build(&self, app: &mut App) {
        init_physics_bridge(app);
        app.add_systems(Update, process_physics_queries);
    }
}

/// Call this from your plugin build() to wire things up.
pub fn init_physics_bridge(app: &mut App) {
    let (tx, rx) = unbounded::<PhysicsQuery>();
    PHYSICS_TX.set(tx).ok(); // make sender available to Py function
    app.insert_resource(PhysicsRx(rx));
}

fn process_physics_queries(receiver: Res<PhysicsRx>, spatial: SpatialQuery) {
    for q in receiver.0.try_iter() {
        match q {
            PhysicsQuery::SegmentCollision2D { seg, reply } => {
                let collided = segments_hits_wall(&spatial, seg);
                let _ = reply.send(collided);
            }
        }
    }
}

fn segments_hits_wall(spatial: &SpatialQuery, seg: Segment2D) -> bool {
    let shape = Collider::cuboid(1.0, 1.0, 1.0);
    let filter = SpatialQueryFilter::from_mask(LayerMask(COLLISION_LAYER_WALL));

    let start = Vec3::new(seg.start.x, 0.5, seg.start.y);
    let end = Vec3::new(seg.end.x, 0.5, seg.end.y);
    let delta = end - start;
    let dist = delta.length();

    if dist <= f32::EPSILON {
        return !spatial
            .shape_intersections(&shape, start, Quaternion::IDENTITY, &filter)
            .is_empty();
    }

    matches!(
        spatial.cast_shape(
            &shape,
            start,
            Quaternion::IDENTITY,
            Dir3::new(delta).unwrap(),
            &ShapeCastConfig::from_max_distance(dist),
            &filter,
        ),
        Some(_hit)
    )
}
