use bevy_third_person_camera::*;
use crate::gameplay::movement::*;
use crate::prelude::*;
use super::Player;

pub fn spawn_player(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    commands.spawn(Player)
        .insert(Name::new("player"))
        .insert(StateScoped(InGameplay))
        .insert(ThirdPersonCameraTarget)
        .insert(MovementState::Grounded)
        .insert(MovementSpeed(constants::player::MOVEMENT_SPEED))
        .insert(JumpForce(constants::player::JUMP_FORCE))
        .insert(utils::load_scene(&assets.player_model))
        .insert(physics::PlayerBundle::default())
    ;
}