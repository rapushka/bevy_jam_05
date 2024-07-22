use crate::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Order {
    Input,
    Gameplay,
}

pub(super) struct OrderPlugin;

impl Plugin for OrderPlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_sets(Update, (Order::Input, Order::Gameplay).chain())
        ;
    }
}