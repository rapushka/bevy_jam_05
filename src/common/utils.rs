use bevy::prelude::Image;
use bevy::sprite::SpriteBundle;
use crate::prelude::{default, Handle, Scene, SceneBundle};

pub mod extensions;

pub fn load_scene(asset: &Handle<Scene>) -> SceneBundle {
    SceneBundle { scene: asset.clone(), ..default() }
}

pub fn load_image(asset: &Handle<Image>) -> SpriteBundle {
    SpriteBundle {
        texture: asset.clone(),
        ..default()
    }
}