use crate::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(
                LoadingState::new(AppState::Bootstrap)
                    .load_collection::<Assets>()
                    .continue_to_state(AppState::MainMenu)
            )
        ;
    }
}

#[derive(AssetCollection, Resource)]
struct Assets {
    #[asset(path = "models/Robot.gltf#Scene0")]
    player_model: Handle<Scene>,
}