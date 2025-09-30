use bevy::prelude::*;

use crate::core::CTFConfig;
use crate::flag::CapturePointBundle;
use crate::team::TeamId;

use super::components::FlagBundle;
use super::visual::{CapturePointGraphicsAssets, FlagGraphicsAssets};

pub fn spawn_flags_headless(mut commands: Commands, config: Res<CTFConfig>) {
    for (team, positions) in [
        (TeamId::Red, &config.red_team_flag_positions),
        (TeamId::Blue, &config.blue_team_flag_positions),
    ] {
        for (i, &position) in positions.iter().enumerate() {
            let flag_name = match team {
                TeamId::Blue => format!("Blue Flag {}", i + 1),
                TeamId::Red => format!("Red Flag {}", i + 1),
            };

            commands.spawn((FlagBundle::new(
                &flag_name,
                team,
                Vec3::new(position.0, 0.0, position.1),
            ),));
        }
    }
}

pub fn spawn_flags(
    mut commands: Commands,
    flag_graphics: Res<FlagGraphicsAssets>,
    config: Res<CTFConfig>,
) {
    for (team, positions) in [
        (TeamId::Red, &config.red_team_flag_positions),
        (TeamId::Blue, &config.blue_team_flag_positions),
    ] {
        for (i, &position) in positions.iter().enumerate() {
            let flag_name = match team {
                TeamId::Blue => format!("Blue Flag {}", i + 1),
                TeamId::Red => format!("Red Flag {}", i + 1),
            };

            commands.spawn((
                FlagBundle::new(&flag_name, team, Vec3::new(position.0, 0.0, position.1)),
                Mesh3d(flag_graphics.mesh.clone()),
                MeshMaterial3d(match team {
                    TeamId::Blue => flag_graphics.blue_material.clone(),
                    TeamId::Red => flag_graphics.red_material.clone(),
                }),
            ));
        }
    }
}

pub fn spawn_capture_points_headless(mut commands: Commands, config: Res<CTFConfig>) {
    for (team, positions) in [
        (TeamId::Red, &config.red_team_capture_point_positions),
        (TeamId::Blue, &config.blue_team_capture_point_positions),
    ] {
        for (i, &position) in positions.iter().enumerate() {
            let name = match team {
                TeamId::Blue => format!("Blue Capture Point {}", i + 1),
                TeamId::Red => format!("Red Capture Point {}", i + 1),
            };

            commands.spawn(CapturePointBundle::new(
                &name,
                team,
                Vec3::new(position.0, 0.0, position.1),
            ));
        }
    }
}

pub fn spawn_capture_points(
    mut commands: Commands,
    capture_point_graphics: Res<CapturePointGraphicsAssets>,
    config: Res<CTFConfig>,
) {
    for (team, positions) in [
        (TeamId::Red, &config.red_team_capture_point_positions),
        (TeamId::Blue, &config.blue_team_capture_point_positions),
    ] {
        for (i, &position) in positions.iter().enumerate() {
            let name = match team {
                TeamId::Blue => format!("Blue Capture Point {}", i + 1),
                TeamId::Red => format!("Red Capture Point {}", i + 1),
            };

            commands.spawn((
                CapturePointBundle::new(&name, team, Vec3::new(position.0, 0.0, position.1)),
                Mesh3d(capture_point_graphics.mesh.clone()),
                MeshMaterial3d(match team {
                    TeamId::Red => capture_point_graphics.blue_material.clone(),
                    TeamId::Blue => capture_point_graphics.red_material.clone(),
                }),
            ));
        }
    }
}
