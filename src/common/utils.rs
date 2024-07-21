use crate::prelude::{default, Handle, Scene, SceneBundle};

pub fn load_scene(asset: &Handle<Scene>) -> SceneBundle {
    SceneBundle { scene: asset.clone(), ..default() }
}