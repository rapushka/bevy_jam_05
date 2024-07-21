use crate::prelude::{ComputedStates, Res, State, States};

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum AppState {
    #[default]
    Bootstrap,
    MainMenu,
    Gameplay {
        paused: bool,
    },
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct InGameplay;

impl ComputedStates for InGameplay {
    type SourceStates = AppState;

    fn compute(sources: AppState) -> Option<InGameplay> {
        match sources {
            AppState::Gameplay { .. } => Some(InGameplay),
            _ => None,
        }
    }
}