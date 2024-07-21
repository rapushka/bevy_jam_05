use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

use crate::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Bootstrap), spawn_camera)

        // TODO: lock cursor?
        // .add_systems(OnEnter(AppState::Gameplay { paused: true }), unlock_camera)
        // .add_systems(OnEnter(AppState::Gameplay { paused: false }), lock_camera)

        // .add_systems(OnExit(InGameplay), unlock_camera)
        ;
    }
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn("camera".as_name())
        .insert(Camera2dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(IsDefaultUiCamera)
    ;
}