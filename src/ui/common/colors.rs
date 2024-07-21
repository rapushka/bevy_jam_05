use bevy::prelude::*;

pub fn default_button() -> BackgroundColor { Srgba::rgb(0.93, 0.71, 0.32).into() }
pub fn hovered_button() -> BackgroundColor { Srgba::rgb(0.9, 0.81, 0.54).into() }
pub fn pressed_button() -> BackgroundColor { Srgba::rgb(0.89, 0.58, 0.28).into() }

pub fn default_text() -> Color { Srgba::rgb(0.30, 0.25, 0.22).into() }
pub fn light_text() -> Color { Srgba::rgb(0.9, 0.9, 0.9).into() }