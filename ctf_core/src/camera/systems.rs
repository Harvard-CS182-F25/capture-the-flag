use std::f32::consts::PI;

use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_isometry(Isometry3d {
            rotation: { Quat::from_axis_angle(Vec3::Y, PI / 2.0) },
            translation: Vec3::new(0.0, 10.0, 0.0).into(),
        })
        .looking_at(Vec3::ZERO, Vec3::NEG_Z),
        Projection::from(OrthographicProjection {
            scale: -0.1,
            ..OrthographicProjection::default_3d()
        }),
    ));
}
