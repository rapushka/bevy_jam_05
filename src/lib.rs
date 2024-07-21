use prelude::*;
use crate::camera::CameraPlugin;
use crate::custom_default_plugin::*;
use crate::debug::DebugPlugin;
use crate::gameplay::GameplayPlugin;
use crate::input::InputPlugin;
use crate::physics::PhysicsPlugin;
use crate::ui::UiPlugin;

mod debug;
mod assets_loading;
mod prelude;
mod custom_default_plugin;
mod infrastructure;
mod ui;
mod common;
mod camera;
mod gameplay;
pub mod input;
pub mod physics;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // # Dependencies
            .add_plugins((
                DefaultPluginWithCustoms,
                AssetLoadingPlugin,
            ))

            // # Mine
            .add_plugins((
                DebugPlugin,
                InfrastructurePlugin,
                UiPlugin,
                CameraPlugin,
                GameplayPlugin,
                InputPlugin,
                PhysicsPlugin,
            ))
        ;
    }
}