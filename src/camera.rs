use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_third_person_camera::*;

use crate::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Bootstrap), spawn_camera)

            .add_systems(OnEnter(InGameplay), lock_camera)

            .add_systems(OnEnter(AppState::Gameplay { paused: true }), lock_camera)
            .add_systems(OnExit(AppState::Gameplay { paused: false }), unlock_camera)

            .add_systems(OnExit(InGameplay), unlock_camera)
        ;
    }
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn((
        ThirdPersonCamera {
            cursor_lock_toggle_enabled: false,
            cursor_lock_active: false,
            sensitivity: Vec2::new(5.0, 5.0),
            zoom: Zoom::new(3.0, 10.0),
            offset_enabled: true,
            offset: Offset::new(0.0, 1.0),
            ..default()
        },
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        IsDefaultUiCamera,
    ));
}

fn lock_camera(
    windows: Query<&mut Window, With<PrimaryWindow>>,
    cameras: Query<&mut ThirdPersonCamera>,
) {
    set_camera_locked(cameras, windows, true);
}

fn unlock_camera(
    cameras: Query<&mut ThirdPersonCamera>,
    windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    set_camera_locked(cameras, windows, false);
}

fn set_camera_locked(
    mut cameras: Query<&mut ThirdPersonCamera>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    value: bool,
) {
    for mut camera in cameras.iter_mut() {
        camera.active = value;
        camera.cursor_lock_active = value;

        let mut window = windows.get_single_mut().unwrap();
        window.cursor.visible = !value;
        window.cursor.grab_mode = if value { CursorGrabMode::Locked } else { CursorGrabMode::None };
    }
}