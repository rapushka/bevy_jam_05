use crate::gameplay::movement::*;
use crate::prelude::*;
use super::Player;

pub fn spawn_player(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    let transform = Transform::from_translation(constants::player::SPAWN_POSITION)
        .with_scale(Vec3::splat(constants::player::SIZE));

    commands.spawn(Player)
        .insert("player".as_name())
        .insert(StateScoped(InGameplay))
        .insert(MovementSpeed(constants::player::MOVEMENT_SPEED))
        .insert(utils::load_sprite_atlas(&assets.player))
        .insert(transform)
    ;
}