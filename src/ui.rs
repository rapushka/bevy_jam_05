use crate::prelude::*;
use crate::ui::common::interactions::InteractionsPlugin;
use crate::ui::gameplay_hud::GameplayHudPlugin;
use crate::ui::main_menu::*;

pub mod create;
pub mod common;
mod main_menu;
mod gameplay_hud;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                InteractionsPlugin,
                MainMenuPlugin,
                GameplayHudPlugin,
            ))
        ;
    }
}