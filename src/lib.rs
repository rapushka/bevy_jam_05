use prelude::*;

use crate::custom_default_plugin::DefaultPluginWithCustoms;

mod prelude;
mod custom_default_plugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPluginWithCustoms)
        ;
    }
}

