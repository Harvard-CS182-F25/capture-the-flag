use bevy::prelude::*;

use crate::team::TeamId;

use super::components::{CapturePointBundle, FlagBundle};
use super::visual::{CapturePointGraphicsAssets, FlagGraphicsAssets};

pub fn spawn_flags(
    mut commands: Commands,
    flag_graphics: Res<FlagGraphicsAssets>,
    capture_point_graphics: Res<CapturePointGraphicsAssets>,
) {
    commands.spawn(FlagBundle::new(
        "Blue Flag",
        TeamId::Blue,
        Vec3::new(-5.0, 0.0, 0.0),
        &flag_graphics,
    ));

    commands.spawn(FlagBundle::new(
        "Blue Flag",
        TeamId::Blue,
        Vec3::new(5.0, 0.0, 0.0),
        &flag_graphics,
    ));

    commands.spawn(FlagBundle::new(
        "Red Flag",
        TeamId::Red,
        Vec3::new(0.0, 0.0, 5.0),
        &flag_graphics,
    ));

    commands.spawn(FlagBundle::new(
        "Red Flag",
        TeamId::Red,
        Vec3::new(0.0, 0.0, -5.0),
        &flag_graphics,
    ));

    commands.spawn(CapturePointBundle::new(
        "Blue Capture Point",
        TeamId::Blue,
        Vec3::new(-10.0, 0.0, 0.0),
        &capture_point_graphics,
    ));

    commands.spawn(CapturePointBundle::new(
        "Blue Capture Point",
        TeamId::Blue,
        Vec3::new(10.0, 0.0, 0.0),
        &capture_point_graphics,
    ));

    commands.spawn(CapturePointBundle::new(
        "Red Capture Point",
        TeamId::Red,
        Vec3::new(0.0, 0.0, -10.0),
        &capture_point_graphics,
    ));

    commands.spawn(CapturePointBundle::new(
        "Red Capture Point",
        TeamId::Red,
        Vec3::new(0.0, 0.0, 10.0),
        &capture_point_graphics,
    ));
}
