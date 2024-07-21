use crate::prelude::*;

pub mod constants;

#[derive(Event)]
pub struct Clicked;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Clicked>()
        ;
    }
}