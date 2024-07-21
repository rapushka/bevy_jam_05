use crate::input::binds::PAUSE_GAME;
use crate::prelude::*;
pub use self::events::*;

mod events;
mod binds;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                pause_game
            ).run_if(in_state(InGameplay)))
        ;
    }
}

fn pause_game(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(PAUSE_GAME) {
        commands.trigger(PauseGame);
    }
}
