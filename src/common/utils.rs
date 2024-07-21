use crate::prelude::*;

pub mod extensions;

pub fn load_image(asset: &Handle<Image>) -> SpriteBundle {
    SpriteBundle {
        texture: asset.clone(),
        ..default()
    }
}