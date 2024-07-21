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
        .insert(MovementSpeed(constants::player::MOVEMENT_SPEED))
        .insert(utils::load_image(&assets.player))
    ;
}