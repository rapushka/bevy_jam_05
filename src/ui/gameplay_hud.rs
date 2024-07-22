use crate::prelude::*;
use self::pause_menu::*;

mod pause_menu;

pub struct GameplayHudPlugin;

impl Plugin for GameplayHudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PauseMenuPlugin)

            .add_systems(OnEnter(InGameplay), create_gameplay_hud)
        ;
    }
}

fn create_gameplay_hud(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    let name = "crosshair";
    commands.spawn(to_name(name))
        .insert(utils::load_sprite_atlas(&assets.crosshair))
    ;
    println!("spawned crosshair");
}

fn to_name(name: &'static str) -> Name {
    Name::new(name)
}