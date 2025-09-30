use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    agent::COLLISION_LAYER_AGENT,
    wall::{COLLISION_LAYER_WALL, WallGraphicsAssets},
};

pub fn spawn_walls_headless(mut commands: Commands) {
    let collider = Collider::cuboid(1.0, 5.0, 20.0);
    let collision_layer = CollisionLayers::new(
        LayerMask(COLLISION_LAYER_WALL),
        LayerMask(COLLISION_LAYER_AGENT | COLLISION_LAYER_WALL),
    );

    commands.spawn((
        Name::new("Wall 1"),
        Transform::from_xyz(-10.0, 2.5, 0.0),
        RigidBody::Static,
        collider.clone(),
        collision_layer,
    ));

    commands.spawn((
        Name::new("Wall 2"),
        Transform::from_xyz(10.0, 2.5, 0.0),
        RigidBody::Static,
        collider.clone(),
        collision_layer,
    ));

    commands.spawn((
        Name::new("Wall 3"),
        Transform::from_xyz(0.0, 2.5, -10.0)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
        RigidBody::Static,
        collider.clone(),
        collision_layer,
    ));

    commands.spawn((
        Name::new("Wall 4"),
        Transform::from_xyz(0.0, 2.5, 10.0)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
        RigidBody::Static,
        collider.clone(),
        collision_layer,
    ));
}

pub fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    graphics: Res<WallGraphicsAssets>,
) {
    let wall_mesh = meshes.add(Cuboid::new(1.0, 5.0, 20.0));
    let collider = Collider::cuboid(1.0, 5.0, 20.0);
    let collision_layer = CollisionLayers::new(
        LayerMask(COLLISION_LAYER_WALL),
        LayerMask(COLLISION_LAYER_AGENT | COLLISION_LAYER_WALL),
    );

    commands.spawn((
        Name::new("Wall 1"),
        Mesh3d(wall_mesh.clone()),
        MeshMaterial3d(graphics.material.clone()),
        Transform::from_xyz(-10.0, 2.5, 0.0),
        RigidBody::Static,
        collider.clone(),
        collision_layer,
    ));

    commands.spawn((
        Name::new("Wall 2"),
        Mesh3d(wall_mesh.clone()),
        MeshMaterial3d(graphics.material.clone()),
        Transform::from_xyz(10.0, 2.5, 0.0),
        RigidBody::Static,
        collider.clone(),
        collision_layer,
    ));

    commands.spawn((
        Name::new("Wall 3"),
        Mesh3d(wall_mesh.clone()),
        MeshMaterial3d(graphics.material.clone()),
        Transform::from_xyz(0.0, 2.5, -10.0)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
        RigidBody::Static,
        collider.clone(),
        collision_layer,
    ));

    commands.spawn((
        Name::new("Wall 4"),
        Mesh3d(wall_mesh),
        MeshMaterial3d(graphics.material.clone()),
        Transform::from_xyz(0.0, 2.5, 10.0)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
        RigidBody::Static,
        collider.clone(),
        collision_layer,
    ));
}
