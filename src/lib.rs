use prelude::*;

use crate::custom_default_plugin::*;

mod assets_loading;
mod prelude;
mod custom_default_plugin;
mod infrastructure;

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
                InfrastructurePlugin,
            ))
        ;
    }
}

