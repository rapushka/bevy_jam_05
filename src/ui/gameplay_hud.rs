use crate::prelude::*;
use self::pause_menu::*;

mod pause_menu;

pub struct GameplayHudPlugin;

impl Plugin for GameplayHudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PauseMenuPlugin)
        ;
    }
}