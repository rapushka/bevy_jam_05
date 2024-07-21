use crate::prelude::*;
use super::create;

#[derive(Component)]
struct PlayButton;
#[derive(Component)]
struct QuitButton;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)

            .add_systems(Update, (
                on_play_button_clicked,
                on_quit_button_clicked,
            ).run_if(in_state(InGameplay)))
        ;
    }
}

fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Name::new("main menu"))
        .insert(NodeBundle {
            style: super::common::styles::main_menu(),
            ..default()
        })
        .with_children(|parent| {
            create::title(&asset_server, parent, "Re:Cycles");
            create::button(&asset_server, parent, "Play", PlayButton);
            create::button(&asset_server, parent, "Quit", QuitButton);
        })
    ;
}

fn on_play_button_clicked() {}
fn on_quit_button_clicked() {}