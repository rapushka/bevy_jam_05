use crate::prelude::*;

type AppStateTransition = StateTransitionEvent<AppState>;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, log_state_transition.run_if(on_event::<AppStateTransition>()))

        ;
    }
}

fn log_state_transition(
    mut event: EventReader<AppStateTransition>,
) {
    for event in event.read() {
        println!("[debug] state transition: {:?} -> {:?}", event.exited, event.entered);
    }
}