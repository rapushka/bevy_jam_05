use crate::gameplay::movement::MovementDirection;
use crate::input::binds::walk;
use crate::prelude::*;

#[derive(Component)]
pub struct InputReceiver;

pub(super) fn read_movement_input(
    mut receivers: Query<&mut MovementDirection, With<InputReceiver>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec2::ZERO;

    if input.pressed(walk::UP) {
        direction += Vec2::Y;
    }
    if input.pressed(walk::LEFT) {
        direction += -Vec2::X;
    }
    if input.pressed(walk::DOWN) {
        direction += -Vec2::Y;
    }
    if input.pressed(walk::RIGHT) {
        direction += Vec2::X;
    }

    direction = direction.normalize_or_zero();

    for mut receiver in receivers.iter_mut() {
        receiver.0 = direction;
    }
} 