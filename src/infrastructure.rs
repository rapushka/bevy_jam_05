pub use self::app_state::*;
use crate::prelude::*;

mod app_state;

pub struct InfrastructurePlugin;

impl Plugin for InfrastructurePlugin {
    fn build(&self, app: &mut App) {
        app
            .enable_state_scoped_entities::<AppState>()
            .enable_state_scoped_entities::<InGameplay>()
            .init_state::<AppState>()
            .add_computed_state::<InGameplay>()
        ;
    }
}

