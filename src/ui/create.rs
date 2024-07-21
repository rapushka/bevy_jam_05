use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::hierarchy::ChildBuilder;

use common::styles;

use crate::prelude::*;
use crate::ui::common;
use crate::ui::common::colors;

pub fn title(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    title_text: &str,
) {
    parent.spawn(NodeBundle { style: styles::title(), ..default() })
        .with_children(|parent| {
            text(asset_server, title_text, parent, 64.0);
        });
}

pub fn button<C>(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    string: &str,
    component: C,
)
where
    C: Component,
{
    button_internal(asset_server, parent, string, component, styles::wide_button());
}

pub fn small_button<C>(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    string: &str,
    component: C,
)
where
    C: Component,
{
    button_internal(asset_server, parent, string, component, styles::narrow_button());
}

fn button_internal<C>(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    string: &str,
    component: C,
    style: Style,
) where
    C: Component,
{
    parent.spawn((
        component,
        ButtonBundle {
            style,
            background_color: colors::default_button(),
            ..default()
        },
    ))
        .with_children(|parent| {
            text(asset_server, string, parent, 32.0);
        });
}

pub fn horizontal_layout(
    parent: &mut ChildBuilder,
    spawn_children: impl FnOnce(&mut ChildBuilder),
) {
    parent.spawn(
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                margin: UiRect::all(Val::Px(25.0)),
                ..default()
            },
            ..default()
        }
    )
        .with_children(spawn_children);
}

pub fn text(
    asset_server: &Res<AssetServer>,
    text: &str,
    parent: &mut ChildBuilder,
    font_size: f32,
) {
    parent.spawn((
        Name::new(format!("text: {text}")),
        text_bundle(asset_server, text, font_size),
    ));
}

pub fn text_bundle(
    asset_server: &Res<AssetServer>,
    text: &str,
    font_size: f32,
) -> TextBundle {
    colored_text_bundle(asset_server, text, font_size, colors::default_text(), ZIndex::default())
}

pub fn light_text_bundle(
    asset_server: &Res<AssetServer>,
    text: &str,
    font_size: f32,
    z_index: ZIndex,
) -> TextBundle {
    colored_text_bundle(asset_server, text, font_size, colors::light_text(), z_index)
}

fn colored_text_bundle(
    asset_server: &Res<AssetServer>,
    text: &str,
    font_size: f32,
    color: Color,
    z_index: ZIndex,
) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![
                TextSection::new(
                    text,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size,
                        color,
                    },
                )],
            justify: JustifyText::Center,
            ..default()
        },
        z_index,
        ..default()
    }
}