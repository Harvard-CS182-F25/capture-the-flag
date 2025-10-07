use bevy::prelude::*;

use crate::wall::{WALL_HEIGHT, WALL_THICKNESS, WallBundle, WallGraphicsAssets};

fn extend_segment(p0: Vec2, p1: Vec2, overlap: f32) -> (Vec2, Vec2) {
    let d = p1 - p0;
    let len = d.length();
    if len <= 1e-6 {
        return (p0, p1);
    }
    let dir = d / len;
    (p0 - dir * overlap, p1 + dir * overlap)
}

pub fn spawn_walls_headless(mut commands: Commands) {
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

    let overlap = WALL_THICKNESS * 0.5 + 0.001;
    for (i, (p0, p1)) in spawn_list.enumerate() {
        let (p0, p1) = extend_segment(p0, p1, overlap);
        commands.spawn((
            Name::new(format!("WallSeg {}", i)),
            WallBundle::new(p0, p1, WALL_THICKNESS),
        ));
    }
}

pub fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    graphics: Res<WallGraphicsAssets>,
) {
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

    let overlap = WALL_THICKNESS * 0.5 + 0.001;
    for (i, (p0, p1)) in spawn_list.enumerate() {
        let (p0, p1) = extend_segment(p0, p1, overlap);
        let len = (p1 - p0).length().max(1e-4);
        let mesh = meshes.add(Cuboid::new(len, WALL_HEIGHT, WALL_THICKNESS));

        commands.spawn((
            Name::new(format!("WallSeg {i}")),
            WallBundle::new(p0, p1, WALL_THICKNESS),
            Mesh3d(mesh),
            MeshMaterial3d(graphics.material.clone()),
        ));
    }
}
