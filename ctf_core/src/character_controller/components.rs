use avian3d::{math::*, prelude::*};
use bevy::prelude::*;

#[derive(Component)]
pub struct CharacterController;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Component)]
pub struct MovementAcceleration(pub Scalar);

#[derive(Component)]
pub struct MovementDampingFactor(pub Scalar);

#[derive(Bundle)]
pub struct MovementBundle {
    movement_acceleration: MovementAcceleration,
    movement_damping_factor: MovementDampingFactor,
}
impl MovementBundle {
    pub const fn new(acceleration: Scalar, damping_factor: Scalar) -> Self {
        Self {
            movement_acceleration: MovementAcceleration(acceleration),
            movement_damping_factor: MovementDampingFactor(damping_factor),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(10.0, 1.0)
    }
}

#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    locked_axes: LockedAxes,
    movement: MovementBundle,
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(
                caster_shape,
                Vector::ZERO,
                Quaternion::default(),
                Dir3::NEG_Y,
            )
            .with_max_distance(0.2),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(mut self, acceleration: Scalar, damping_factor: Scalar) -> Self {
        self.movement = MovementBundle::new(acceleration, damping_factor);
        self
    }

    pub fn _with_locked_axes(mut self, locked_axes: LockedAxes) -> Self {
        self.locked_axes = locked_axes;
        self
    }
}
