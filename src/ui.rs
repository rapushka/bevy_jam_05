use crate::prelude::*;
use crate::ui::main_menu::*;

pub mod create;
pub mod common;
mod main_menu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                MainMenuPlugin,
            ))
        ;
    }
}