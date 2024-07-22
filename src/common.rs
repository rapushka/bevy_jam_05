use crate::prelude::*;
pub use self::order::*;

pub mod constants;
pub mod utils;
pub mod order;

#[derive(Event)]
pub struct Clicked;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(OrderPlugin)

            .add_event::<Clicked>()
        ;
    }
}