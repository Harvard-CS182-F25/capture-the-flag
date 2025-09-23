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
        "Blue Flag 1",
        TeamId::Blue,
        Vec3::new(-35.0, 0.0, -5.0),
        &flag_graphics,
    ));

    commands.spawn(FlagBundle::new(
        "Blue Flag 2",
        TeamId::Blue,
        Vec3::new(-35.0, 0.0, 5.0),
        &flag_graphics,
    ));

    commands.spawn(FlagBundle::new(
        "Red Flag 1",
        TeamId::Red,
        Vec3::new(35.0, 0.0, -5.0),
        &flag_graphics,
    ));

    commands.spawn(FlagBundle::new(
        "Red Flag 2",
        TeamId::Red,
        Vec3::new(35.0, 0.0, 5.0),
        &flag_graphics,
    ));

    commands.spawn(CapturePointBundle::new(
        "Blue Capture Point 1",
        TeamId::Blue,
        Vec3::new(-40.0, 0.0, -5.0),
        &capture_point_graphics,
    ));

    commands.spawn(CapturePointBundle::new(
        "Blue Capture Point 2",
        TeamId::Blue,
        Vec3::new(-40.0, 0.0, 5.0),
        &capture_point_graphics,
    ));

    commands.spawn(CapturePointBundle::new(
        "Red Capture Point 1",
        TeamId::Red,
        Vec3::new(40.0, 0.0, -5.0),
        &capture_point_graphics,
    ));

    commands.spawn(CapturePointBundle::new(
        "Red Capture Point 2",
        TeamId::Red,
        Vec3::new(40.0, 0.0, 5.0),
        &capture_point_graphics,
    ));
}
