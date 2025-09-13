use bevy::prelude::*;

#[derive(Resource)]
pub struct AgentGraphicsAssets {
    pub mesh: Handle<Mesh>,
    pub red_material: Handle<StandardMaterial>,
    pub red_pickup_material: Handle<StandardMaterial>,
    pub blue_material: Handle<StandardMaterial>,
    pub blue_pickup_material: Handle<StandardMaterial>,
}

impl FromWorld for AgentGraphicsAssets {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let mesh = meshes.add(Cuboid::default());

        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let red_material: Handle<StandardMaterial> = materials.add(Color::srgb(1.0, 0.0, 0.0));
        let red_pickup_material: Handle<StandardMaterial> =
            materials.add(Color::srgb(1.0, 0.5, 0.5));

        let blue_material: Handle<StandardMaterial> = materials.add(Color::srgb(0.0, 0.0, 1.0));
        let blue_pickup_material: Handle<StandardMaterial> =
            materials.add(Color::srgb(0.5, 0.5, 1.0));

        Self {
            mesh,
            red_material,
            red_pickup_material,
            blue_material,
            blue_pickup_material,
        }
    }
}
