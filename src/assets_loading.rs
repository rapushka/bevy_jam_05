use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::prelude::*;

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(
                LoadingState::new(AppState::Bootstrap)
                    .load_collection::<GameAssets>()
                    .continue_to_state(AppState::MainMenu)
            )
        ;
    }
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "player/Robot.gltf#Scene0")]
    pub player_model: Handle<Scene>,

    #[asset(path = "player/crosshair.png")]
    pub crosshair: Handle<Image>,
}