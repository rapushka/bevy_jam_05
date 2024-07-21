use std::fmt::Pointer;
use crate::prelude::*;
pub use self::bundles::*;

mod bundles;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::default(),
                #[cfg(debug_assertions)]
                RapierDebugRenderPlugin::default(),
            ))
        ;
    }
}