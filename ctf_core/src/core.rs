use avian3d::prelude::*;
use bevy::prelude::*;

use crate::agent::{AgentPlugin, COLLISION_LAYER_AGENT};
use crate::camera::CameraPlugin;
use crate::character_controller::CharacterControllerPlugin;
use crate::flag::FlagPlugin;
use crate::interaction_range::InteractionRangePlugin;
use crate::team::TeamPlugin;
use crate::wall::WallPlugin;

pub const COLLISION_LAYER_GROUND: u32 = 1 << 2;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct CTFConfig {
    pub red_team_agent_positions: Vec<(f32, f32)>,
    pub blue_team_agent_positions: Vec<(f32, f32)>,
    pub red_team_flag_positions: Vec<(f32, f32)>,
    pub blue_team_flag_positions: Vec<(f32, f32)>,
    pub red_team_capture_point_positions: Vec<(f32, f32)>,
    pub blue_team_capture_point_positions: Vec<(f32, f32)>,
    pub headless: bool,
}

pub struct CTFPlugin {
    pub red_team_agent_positions: Vec<(f32, f32)>,
    pub blue_team_agent_positions: Vec<(f32, f32)>,
    pub red_team_flag_positions: Vec<(f32, f32)>,
    pub blue_team_flag_positions: Vec<(f32, f32)>,
    pub red_team_capture_point_positions: Vec<(f32, f32)>,
    pub blue_team_capture_point_positions: Vec<(f32, f32)>,
    pub headless: bool,
}

impl Plugin for CTFPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AgentPlugin,
            CameraPlugin,
            CharacterControllerPlugin,
            FlagPlugin,
            InteractionRangePlugin,
            TeamPlugin,
            WallPlugin,
        ));
        app.register_type::<CTFConfig>();
        app.insert_resource(CTFConfig {
            red_team_agent_positions: self.red_team_agent_positions.clone(),
            blue_team_agent_positions: self.blue_team_agent_positions.clone(),
            red_team_flag_positions: self.red_team_flag_positions.clone(),
            blue_team_flag_positions: self.blue_team_flag_positions.clone(),
            red_team_capture_point_positions: self.red_team_capture_point_positions.clone(),
            blue_team_capture_point_positions: self.blue_team_capture_point_positions.clone(),
            headless: self.headless,
        });

        app.add_systems(Startup, setup_scene.run_if(|c: Res<CTFConfig>| !c.headless));
        app.add_systems(
            Startup,
            setup_scene_headless.run_if(|c: Res<CTFConfig>| c.headless),
        );
    }
}

fn setup_scene_headless(mut commands: Commands) {
    commands.spawn((
        Name::new("Ground Plane"),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(100.0, 1.0, 100.0)),
        RigidBody::Static,
        Collider::cuboid(100.0, 1.0, 100.0),
        CollisionLayers::new(
            LayerMask(COLLISION_LAYER_GROUND),
            LayerMask(COLLISION_LAYER_AGENT),
        ),
    ));
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_plane_mesh = meshes.add(Plane3d::default());
    let green_material = materials.add(Color::srgb(0.0, 1.0, 0.0));

    commands.spawn((
        Name::new("Ground Plane"),
        Mesh3d(ground_plane_mesh),
        MeshMaterial3d(green_material),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(100.0, 1.0, 100.0)),
        RigidBody::Static,
        Collider::cuboid(100.0, 1.0, 100.0),
        CollisionLayers::new(
            LayerMask(COLLISION_LAYER_GROUND),
            LayerMask(COLLISION_LAYER_AGENT),
        ),
    ));
}
