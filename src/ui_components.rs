pub mod button_effect;
pub mod input_area;

use bevy::prelude::*;

use self::{button_effect::UiButtonEffectPlugin, input_area::UiInputAreaPlugin};

/*
 * Part：插件
 */

pub(super) struct UiComponentsPlugin;

impl Plugin for UiComponentsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Button effect
            .add_plugins(UiButtonEffectPlugin)
            // Input Area
            .add_plugins(UiInputAreaPlugin);
    }
}
