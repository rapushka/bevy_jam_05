use prelude::*;
use crate::camera::CameraPlugin;
use crate::custom_default_plugin::*;
use crate::gameplay::GameplayPlugin;
use crate::ui::UiPlugin;

mod assets_loading;
mod prelude;
mod custom_default_plugin;
mod infrastructure;
mod ui;
mod common;
mod camera;
mod gameplay;

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
                UiPlugin,
                CameraPlugin,
                GameplayPlugin,
            ))

            .add_systems(OnEnter(InGameplay), || { println!("in gameplay") })
            .add_systems(Update, log_state_transition.run_if(on_event::<StateTransitionEvent<AppState>>()))
        ;
    }
}

fn log_state_transition(
    mut event: EventReader<StateTransitionEvent<AppState>>,
) {
    for event in event.read() {
        dbg!(event);
    }
}