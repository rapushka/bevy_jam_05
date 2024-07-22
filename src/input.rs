use crate::gameplay::movement::MovementDirection;
use crate::prelude::*;
pub use self::events::*;
pub use self::movement::*;

mod events;
mod binds;
mod pause;
mod movement;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<MovementDirection>()

            .add_systems(Update, (
                pause::pause_game,
                read_movement_input,
            )
                .run_if(in_state(InGameplay))
                .in_set(Order::Input),
            )
        ;
    }
}

