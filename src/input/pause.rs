use crate::input::binds::PAUSE_GAME;
use crate::input::PauseGame;
use crate::prelude::{ButtonInput, Commands, KeyCode, Res};

pub fn pause_game(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(PAUSE_GAME) {
        commands.trigger(PauseGame);
    }
}