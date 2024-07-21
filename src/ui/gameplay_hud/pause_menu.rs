use crate::prelude::*;
use crate::ui::{common, create};

#[derive(Component)]
struct ContinueButton;

#[derive(Component)]
struct BackButton;

#[derive(Component)]
struct PauseMenu;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(InGameplay), (
                spawn_pause_menu,
                hide_pause_menu,
            ).chain())

            .add_systems(OnEnter(AppState::Gameplay { paused: true }), show_pause_menu)
            .add_systems(OnEnter(AppState::Gameplay { paused: false }), hide_pause_menu)

            .observe(toggle_pause_menu_visibility)
            .observe(on_continue_button_clicked)
            .observe(on_back_button_clicked)
        ;
    }
}

fn spawn_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Name::new("pause menu"))
        .insert(PauseMenu)
        .insert(StateScoped(InGameplay))
        .insert(NodeBundle {
            style: common::styles::main_menu(),
            z_index: common::z_index::PAUSE_MENU,
            ..default()
        })
        .with_children(|parent| {
            create::title(&asset_server, parent, "Paused");
            create::button(&asset_server, parent, "Continue", ContinueButton);
            create::button(&asset_server, parent, "Back to Main Menu", BackButton);
        })
    ;
}

fn toggle_pause_menu_visibility(
    _trigger: Trigger<input::PauseGame>,
    mut pause_menus: Query<&mut Visibility, With<PauseMenu>>,
) {
    for mut visibility in pause_menus.iter_mut() {
        let was_visible = match *visibility {
            Visibility::Inherited => true,
            Visibility::Hidden => false,
            Visibility::Visible => true,
        };

        *visibility = as_visibility(!was_visible);
    }
}

fn show_pause_menu(
    pause_menus: Query<&mut Visibility, With<PauseMenu>>,
) {
    set_pause_menu_visible(pause_menus, true);
}
fn hide_pause_menu(
    pause_menus: Query<&mut Visibility, With<PauseMenu>>,
) {
    set_pause_menu_visible(pause_menus, false);
}

fn set_pause_menu_visible(
    mut pause_menus: Query<&mut Visibility, With<PauseMenu>>,
    value: bool,
) {
    dbg!(value);
    for mut visibility in pause_menus.iter_mut() {
        *visibility = as_visibility(value);
    }
}

fn as_visibility(value: bool) -> Visibility {
    if value { Visibility::Inherited } else { Visibility::Hidden }
}

fn on_continue_button_clicked(
    trigger: Trigger<Clicked>,
    play_buttons: Query<Entity, With<ContinueButton>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let is_play_button = play_buttons.contains(trigger.entity());
    if is_play_button {
        next_state.set(AppState::Gameplay { paused: false });
    }
}
fn on_back_button_clicked(
    trigger: Trigger<Clicked>,
    play_buttons: Query<Entity, With<BackButton>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let is_play_button = play_buttons.contains(trigger.entity());
    if is_play_button {
        next_state.set(AppState::MainMenu);
    }
}
