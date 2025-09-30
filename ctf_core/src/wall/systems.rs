use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    agent::COLLISION_LAYER_AGENT,
    wall::{COLLISION_LAYER_WALL, WallGraphicsAssets},
};

// --- parameters ---
const WALL_HEIGHT: f32 = 5.0; // full height in world units
const WALL_THICK: f32 = 1.0; // full thickness

fn segment_components(p0: Vec2, p1: Vec2) -> (Transform, Collider) {
    // center & orientation on XZ plane (Y up)
    let a = Vec3::new(p0.x, WALL_HEIGHT * 0.5, p0.y);
    let b = Vec3::new(p1.x, WALL_HEIGHT * 0.5, p1.y);
    let dir = b - a;
    let len = dir.length().max(1e-4);
    let center = (a + b) * 0.5;
    // local X axis points along the bar
    let yaw = dir.z.atan2(dir.x);
    let transform = Transform::from_translation(center).with_rotation(Quat::from_rotation_y(yaw));
    let collider = Collider::cuboid(len, WALL_HEIGHT, WALL_THICK);
    (transform, collider)
}

fn wall_layers() -> CollisionLayers {
    CollisionLayers::new(
        LayerMask(COLLISION_LAYER_WALL),
        LayerMask(COLLISION_LAYER_AGENT | COLLISION_LAYER_WALL),
    )
}

// ----------------------------------
// Headless
// ----------------------------------
pub fn spawn_walls_headless(mut commands: Commands) {
    let layers = wall_layers();

    let outer = [
        (Vec2::new(-50.0, 50.0), Vec2::new(50.0, 50.0)),
        (Vec2::new(50.0, 50.0), Vec2::new(50.0, -50.0)),
        (Vec2::new(50.0, -50.0), Vec2::new(-50.0, -50.0)),
        (Vec2::new(-50.0, -50.0), Vec2::new(-50.0, 50.0)),
    ];

    let side_bars = [
        (Vec2::new(-45.0, 45.0), Vec2::new(-45.0, 5.0)),
        (Vec2::new(-45.0, -5.0), Vec2::new(-45.0, -45.0)),
        (Vec2::new(45.0, 45.0), Vec2::new(45.0, 5.0)),
        (Vec2::new(45.0, -5.0), Vec2::new(45.0, -45.0)),
    ];

    // Middle horizontal bars (purple in your plot)
    let middle = [
        (Vec2::new(-10.0, 5.0), Vec2::new(10.0, 5.0)),
        (Vec2::new(-10.0, -5.0), Vec2::new(10.0, -5.0)),
    ];

    // Center diamonds (from your Desmos polygons)
    let diamond_left_edges = [
        (Vec2::new(-5.0, 0.0), Vec2::new(-35.0, 30.0)),
        (Vec2::new(-35.0, -30.0), Vec2::new(-5.0, 0.0)),
        (Vec2::new(-5.0, 0.0), Vec2::new(25.0, -30.0)),
        (Vec2::new(25.0, 20.0), Vec2::new(5.0, 0.0)),
    ];

    let diamond_right_edges = [
        (Vec2::new(5.0, 0.0), Vec2::new(35.0, 30.0)),
        (Vec2::new(35.0, -30.0), Vec2::new(5.0, 0.0)),
        (Vec2::new(5.0, 0.0), Vec2::new(-25.0, 30.0)),
        (Vec2::new(-25.0, -20.0), Vec2::new(-5.0, 0.0)),
    ];

    let spawn_list = outer
        .into_iter()
        .chain(side_bars)
        .chain(middle)
        .chain(diamond_left_edges)
        .chain(diamond_right_edges);

    for (i, (p0, p1)) in spawn_list.enumerate() {
        let (transform, collider) = segment_components(p0, p1);
        commands.spawn((
            Name::new(format!("WallSeg {}", i)),
            RigidBody::Static,
            transform,
            collider,
            layers,
        ));
    }
}

// ----------------------------------
// With Mesh
// ----------------------------------
pub fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    graphics: Res<WallGraphicsAssets>,
) {
    let layers = wall_layers();

    let outer = [
        (Vec2::new(-50.0, 50.0), Vec2::new(50.0, 50.0)),
        (Vec2::new(50.0, 50.0), Vec2::new(50.0, -50.0)),
        (Vec2::new(50.0, -50.0), Vec2::new(-50.0, -50.0)),
        (Vec2::new(-50.0, -50.0), Vec2::new(-50.0, 50.0)),
    ];

    let side_bars = [
        (Vec2::new(-45.0, 45.0), Vec2::new(-45.0, 5.0)),
        (Vec2::new(-45.0, -5.0), Vec2::new(-45.0, -45.0)),
        (Vec2::new(45.0, 45.0), Vec2::new(45.0, 5.0)),
        (Vec2::new(45.0, -5.0), Vec2::new(45.0, -45.0)),
    ];

    // Middle horizontal bars (purple in your plot)
    let middle = [
        (Vec2::new(-10.0, 5.0), Vec2::new(10.0, 5.0)),
        (Vec2::new(-10.0, -5.0), Vec2::new(10.0, -5.0)),
    ];

    // Center diamonds (from your Desmos polygons)
    let diamond_left_edges = [
        (Vec2::new(-5.0, 0.0), Vec2::new(-35.0, 30.0)),
        (Vec2::new(-35.0, -30.0), Vec2::new(-5.0, 0.0)),
        (Vec2::new(-5.0, 0.0), Vec2::new(25.0, -30.0)),
        (Vec2::new(25.0, 20.0), Vec2::new(5.0, 0.0)),
    ];

    let diamond_right_edges = [
        (Vec2::new(5.0, 0.0), Vec2::new(35.0, 30.0)),
        (Vec2::new(35.0, -30.0), Vec2::new(5.0, 0.0)),
        (Vec2::new(5.0, 0.0), Vec2::new(-25.0, 30.0)),
        (Vec2::new(-25.0, -20.0), Vec2::new(-5.0, 0.0)),
    ];

    let spawn_list = outer
        .into_iter()
        .chain(side_bars)
        .chain(middle)
        .chain(diamond_left_edges)
        .chain(diamond_right_edges);

    for (p0, p1) in spawn_list {
        let (transform, collider) = segment_components(p0, p1);

        // Mesh wants full extents
        let len = (p1 - p0).length().max(1e-4);
        let mesh = meshes.add(Cuboid::new(len, WALL_HEIGHT, WALL_THICK));

        commands.spawn((
            RigidBody::Static,
            Mesh3d(mesh),
            MeshMaterial3d(graphics.material.clone()),
            transform,
            collider,
            layers,
        ));
    }
}
