use bevy::prelude::*;

pub fn main_menu() -> Style {
    Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        row_gap: Val::Px(8.0),
        ..default()
    }
}

pub fn gameplay_hud() -> Style {
    Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        padding: UiRect::all(Val::Px(10.0)),
        ..default()
    }
}

pub fn speech_bubble() -> Style {
    Style {
        width: Val::Percent(75.0),
        height: Val::Percent(20.0),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_self: AlignSelf::FlexEnd,
        align_items: AlignItems::Center,
        margin: UiRect {
            left: Val::Auto,
            right: Val::Auto,
            top: Val::Auto,
            bottom: Val::Px(20.0),
        },
        ..default()
    }
}

pub fn wide_button() -> Style {
    button(200.0)
}

pub fn narrow_button() -> Style {
    button(100.0)
}

pub fn button(width: f32) -> Style {
    Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(width),
        height: Val::Px(80.0),
        ..default()
    }
}

pub fn title() -> Style {
    Style {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(300.0),
        height: Val::Px(300.0),
        ..default()
    }
}
