use crate::gameplay::player;
use crate::prelude::*;

pub struct GameLoopPlugin;

impl Plugin for GameLoopPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(InGameplay), (
                player::spawn::spawn_player,
            ))
        ;
    }
}