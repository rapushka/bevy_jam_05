use bevy::asset::AssetMetaCheck;
use crate::prelude::*;

pub struct DefaultPluginWithCustoms;

impl Plugin for DefaultPluginWithCustoms {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
            )
        ;
    }
}