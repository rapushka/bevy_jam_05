use bevy_third_person_camera::ThirdPersonCameraTarget;
use crate::gameplay::movement::{JumpForce, MovementSpeed, MovementState};
use crate::prelude::*;
use super::Player;

pub fn spawn_player(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    println!("player spawned");
    
    commands.spawn(Player)
        .insert(Name::new("player"))
        .insert(StateScoped(InGameplay))
        .insert(ThirdPersonCameraTarget)
        .insert(MovementState::Grounded)
        .insert(MovementSpeed(constants::PLAYER_MOVEMENT_SPEED))
        .insert(JumpForce(constants::JUMP_FORCE))
        .insert(SceneBundle {
            scene: assets.player_model.clone(),
            transform: Transform::from_xyz(-1.0, 0.0, 0.0),
            ..default()
        })
    // .insert(LockedAxes::ROTATION_LOCKED)
    // .insert(MoveDirection(Vec3::ZERO))
    // .insert(KinematicCharacterController::default())
    // .insert(RigidBody::Dynamic)
    // .insert(Collider::capsule(Vec3::Y, Vec3::Y * 2.0, 1.0))
    // .insert(GravityScale(GRAVITY_SCALE))
    // .insert(ColliderMassProperties::Mass(PLAYER_MASS))
    // .insert(Velocity::default())
    // .insert(ExternalImpulse::default())
    ;
}