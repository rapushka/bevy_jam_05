pub use self::app_state::AppState;
use crate::prelude::*;

mod app_state;

pub struct InfrastructurePlugin;

impl Plugin for InfrastructurePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<AppState>()
        ;
    }
}

