use bevy::prelude::*;

pub const PAUSE_GAME: KeyCode = KeyCode::Escape;

pub mod walk {
    use crate::prelude::*;

    pub const UP: KeyCode = KeyCode::KeyW;
    pub const LEFT: KeyCode = KeyCode::KeyA;
    pub const DOWN: KeyCode = KeyCode::KeyS;
    pub const RIGHT: KeyCode = KeyCode::KeyD;
}