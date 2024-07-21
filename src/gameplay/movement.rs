use bevy::prelude::Component;

#[derive(Component)]
pub struct MovementSpeed(pub f32);

#[derive(Component)]
pub struct JumpForce(pub f32);

#[derive(Component)]
pub enum MovementState {
    Grounded,
    Airborne {
        jumps_made: u8,
    },
}