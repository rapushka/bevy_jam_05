use crate::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    locked_axes: LockedAxes,
    rigid_body: RigidBody,
    spatial_bundle: SpatialBundle,
    controller: KinematicCharacterController,
    collider: Collider,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        let height = Vec3::Y * constants::player::COLLIDER_HEIGHT;
        let radius = constants::player::COLLIDER_RADIUS;

        Self {
            locked_axes: LockedAxes::ROTATION_LOCKED,
            rigid_body: RigidBody::KinematicPositionBased,
            spatial_bundle: SpatialBundle::default(),
            controller: KinematicCharacterController::default(),
            collider: Collider::capsule(Vec3::Y, height, radius),
            // TODO: remove?
            // moveDirection: MoveDirection(Vec3::ZERO),
            // KinematicCharacterController::default(),
            // RigidBody::Dynamic,
            // Collider::capsule(Vec3::Y, Vec3::Y * 2.0, 1.0),
            // GravityScale(GRAVITY_SCALE),
            // ColliderMassProperties::Mass(PLAYER_MASS),
            // Velocity::default(),
            // ExternalImpulse::default(),
        }
    }
}