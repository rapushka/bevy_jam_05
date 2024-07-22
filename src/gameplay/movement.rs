use crate::prelude::*;

#[derive(Component)]
pub struct MovementSpeed(pub f32);
#[derive(Component, Reflect)]
pub struct MovementDirection(pub Vec2);

pub(super) fn move_entities(
    mut movables: Query<(&mut Transform, &MovementDirection, &MovementSpeed)>,
    time: Res<Time>,
) {
    for (mut transform, direction, speed) in movables.iter_mut() {
        let movement = direction.0 * speed.0 * time.delta_seconds();
        transform.translation += movement.extend(0.0);
    }
}