#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use bevy_jam_05::GamePlugin;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}