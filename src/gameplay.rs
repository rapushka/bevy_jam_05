use crate::gameplay::game_loop::GameLoopPlugin;
use crate::prelude::*;

pub mod player;
mod game_loop;
pub mod movement;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(GameLoopPlugin)
        ;
    }
}