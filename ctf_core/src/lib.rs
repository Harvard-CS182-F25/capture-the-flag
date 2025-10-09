use avian3d::{math::Quaternion, prelude::*};
use bevy::prelude::*;

pub mod agent;
pub mod camera;
pub mod character_controller;
pub mod core;
pub mod debug;
pub mod flag;
pub mod interaction_range;
pub mod team;
pub mod wall;

#[derive(Debug, Clone, Copy)]
pub struct Segment2D {
    pub start: Vec2,
    pub end: Vec2,
}

pub fn segment_hits_wall_flag_or_capture_point(
    spatial: &SpatialQuery,
    seg: Segment2D,
    team_id: team::TeamId,
) -> bool {
    let shape = Collider::cuboid(1.0, 1.0, 1.0);

    // 1) Walls: simple mask
    let block_mask = match team_id {
        team::TeamId::Red => wall::COLLISION_LAYER_WALL | flag::COLLISION_LAYER_CAMP_BLOCK_RED,
        team::TeamId::Blue => wall::COLLISION_LAYER_WALL | flag::COLLISION_LAYER_CAMP_BLOCK_BLUE,
    };
    let filter = SpatialQueryFilter::from_mask(LayerMask(block_mask));

    let start = Vec3::new(seg.start.x, 0.5, seg.start.y);
    let end = Vec3::new(seg.end.x, 0.5, seg.end.y);
    let delta = end - start;
    let dist = delta.length();
    let rot = Quaternion::IDENTITY;

    if !dist.is_finite() || dist <= 1e-4 {
        return !spatial
            .shape_intersections(&shape, start, rot, &filter)
            .is_empty();
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
