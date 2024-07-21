use crate::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
        ;
    }
}

fn spawn_main_menu() {
    
}