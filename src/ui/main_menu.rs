use crate::prelude::*;
use super::{common, create};

#[derive(Component)]
struct PlayButton;
#[derive(Component)]
struct QuitButton;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)

            .observe(on_play_button_clicked)
            .observe(on_quit_button_clicked)
        ;
    }
}

fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Name::new("main menu"))
        .insert(StateScoped(AppState::MainMenu))
        .insert(NodeBundle {
            style: common::styles::main_menu(),
            z_index: common::z_index::MAIN_MENU,
            ..default()
        })
        .with_children(|parent| {
            create::title(&asset_server, parent, "Re:Cycles");
            create::button(&asset_server, parent, "Play", PlayButton);
            create::button(&asset_server, parent, "Quit", QuitButton);
        })
    ;
}

fn on_play_button_clicked(
    trigger: Trigger<Clicked>,
    play_buttons: Query<Entity, With<PlayButton>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let is_play_button = play_buttons.contains(trigger.entity());
    if is_play_button {
        next_state.set(AppState::Gameplay { paused: false });
    }
}
fn on_quit_button_clicked(
    trigger: Trigger<Clicked>,
    quit_buttons: Query<Entity, With<QuitButton>>,
    mut app_exit_event: EventWriter<AppExit>,
) {
    let is_quit_button = quit_buttons.contains(trigger.entity());
    if is_quit_button {
        app_exit_event.send(AppExit::Success);
    }
}