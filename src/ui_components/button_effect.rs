use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/*
 * Part：插件
 */

pub(super) struct UiButtonEffectPlugin;

impl Plugin for UiButtonEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_effect_system);
    }
}

#[allow(clippy::type_complexity)]
fn button_effect_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &ButtonColorCollection,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    interaction_query.iter_mut().for_each(
        |(interaction, mut mut_background_color, mut mut_border_color, button_color_collection)| {
            let collection = match interaction {
                Interaction::Pressed => button_color_collection.pressed,
                Interaction::Hovered => button_color_collection.hovered,
                Interaction::None => button_color_collection.normal,
            };
            *mut_border_color = collection.border;
            *mut_background_color = collection.background;
        },
    );
}

/*
 * Part：颜色类型
 */

#[derive(Debug, Clone, Copy, Component, Serialize, Deserialize)]
pub struct ButtonColorCollection {
    pub normal: ButtonColors,
    pub hovered: ButtonColors,
    pub pressed: ButtonColors,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ButtonColors {
    pub background: BackgroundColor,
    pub border: BorderColor,
}

impl ButtonColorCollection {
    pub const DEFAULT: Self = Self::new(
        Color::Rgba {
            red: 0.85,
            green: 0.85,
            blue: 0.85,
            alpha: 1.0,
        },
        Color::BLACK,
        Color::Rgba {
            red: 0.7,
            green: 0.7,
            blue: 0.7,
            alpha: 1.0,
        },
        Color::BLACK,
        Color::Rgba {
            red: 0.5,
            green: 0.5,
            blue: 0.5,
            alpha: 1.0,
        },
        Color::BLACK,
    );

    pub const fn new(
        normal_background: Color,
        normal_border: Color,
        hovered_background: Color,
        hovered_border: Color,
        pressed_background: Color,
        pressed_border: Color,
    ) -> Self {
        Self {
            normal: ButtonColors {
                background: BackgroundColor(normal_background),
                border: BorderColor(normal_border),
            },
            hovered: ButtonColors {
                background: BackgroundColor(hovered_background),
                border: BorderColor(hovered_border),
            },
            pressed: ButtonColors {
                background: BackgroundColor(pressed_background),
                border: BorderColor(pressed_border),
            },
        }
    }

    pub const fn from_background(
        normal_background: Color,
        hovered_background: Color,
        pressed_background: Color,
    ) -> Self {
        Self::new(
            normal_background,
            Self::DEFAULT.normal.background.0,
            hovered_background,
            Self::DEFAULT.hovered.background.0,
            pressed_background,
            Self::DEFAULT.pressed.background.0,
        )
    }
}

impl Default for ButtonColorCollection {
    fn default() -> Self {
        Self::DEFAULT
    }
}
