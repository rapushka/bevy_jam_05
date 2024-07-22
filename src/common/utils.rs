use crate::prelude::*;

pub mod extensions;

pub fn load_sprite_atlas(asset: &Handle<Image>) -> SpriteBundle {
    SpriteBundle {
        texture: asset.clone(),
        ..default()
    }
}