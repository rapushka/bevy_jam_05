use crate::prelude::*;

pub mod constants;
pub mod utils;

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