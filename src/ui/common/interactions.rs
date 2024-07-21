use crate::prelude::*;
use super::colors;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                visualise_interaction_with_buttons,
                click_on_pressed_button,
            ))
        ;
    }
}

fn visualise_interaction_with_buttons(
    mut buttons: Query<(&Interaction, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, mut background_color) in buttons.iter_mut() {
        let color = match *interaction {
            Interaction::Pressed => colors::pressed_button(),
            Interaction::Hovered => colors::hovered_button(),
            Interaction::None => colors::default_button(),
        };

        *background_color = color.into()
    }
}

fn click_on_pressed_button(
    mut commands: Commands,
    buttons: Query<(Entity, &Interaction), Changed<Interaction>>,
) {
    for (entity, interaction) in buttons.iter() {
        if *interaction == Interaction::Pressed {
            commands.trigger_targets(Clicked, entity);
        }
    }
}