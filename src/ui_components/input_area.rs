use std::time::{Duration, SystemTime};

use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};

/*
 * Part：插件
 */

pub(super) struct UiInputAreaPlugin;

impl Plugin for UiInputAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, input_area_focus_system)
            .add_systems(PreUpdate, input_area_unfocus_system)
            .add_systems(Update, input_area_input_system);
    }
}

/*
 * Part：系统
 */

#[allow(clippy::type_complexity)]
fn input_area_focus_system(
    mut commands: Commands,
    input_query_previous: Query<Entity, With<InputAreaFocused>>,
    input_query: Query<
        (Entity, &Interaction, &InputArea),
        (Changed<Interaction>, Without<InputAreaFocused>),
    >,
) {
    input_query
        .iter()
        .filter(|(_, interaction, _)| matches!(interaction, Interaction::Pressed))
        .take(1)
        .for_each(|(entity, _, _)| {
            input_query_previous.iter().for_each(|entity| {
                commands.get_or_spawn(entity).remove::<InputAreaFocused>();
            });
            commands.get_or_spawn(entity).insert(InputAreaFocused {
                last_active_time: SystemTime::now(),
            });
        });
}

fn input_area_unfocus_system(
    mut commands: Commands,
    query_area: Query<(Entity, &InputAreaFocused)>,
) {
    let time = SystemTime::now();
    for (entity, focused) in query_area.iter() {
        if time
            .duration_since(focused.last_active_time)
            .ok()
            .filter(|duration| duration < &Duration::from_secs(10))
            .is_none()
        {
            commands.get_or_spawn(entity).remove::<InputAreaFocused>();
        }
    }
}

fn input_area_input_system(
    mut query_area: Query<(&Children, &InputArea, &mut InputAreaFocused)>,
    mut query_text: Query<&mut Text>,
    mut keyboard_events: EventReader<KeyboardInput>,
) {
    for event in keyboard_events.read() {
        let ButtonState::Pressed = event.state else {
            continue;
        };
        for (children, area, mut focused) in query_area.iter_mut() {
            for child_entity in children.iter() {
                let Ok(mut text) = query_text.get_mut(*child_entity) else {
                    continue;
                };
                // 更新时间
                focused.last_active_time = SystemTime::now();
                // 操作：输入
                if let Key::Character(smol_str) = &event.logical_key {
                    let Some(key_char) = smol_str.chars().next() else {
                        continue;
                    };
                    // 该字符是否可以输入？
                    if !area.area_type.is_match(key_char) {
                        continue;
                    }
                    text.sections[0].value.push(key_char);
                }
                // 操作：退格
                if let Key::Backspace = &event.logical_key {
                    text.sections[0].value.pop();
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Component)]
pub struct InputArea {
    pub area_type: InputAreaType,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, Default, Component)]
pub enum InputAreaType {
    #[default]
    All,
    Number,
    NumAlpha,
    IpAddress,
    MacAddress,
}

impl InputAreaType {
    pub const fn is_match(&self, ch: char) -> bool {
        match self {
            InputAreaType::All => true,
            InputAreaType::Number => ch.is_ascii_digit(),
            InputAreaType::NumAlpha => ch.is_ascii_alphanumeric(),
            InputAreaType::IpAddress => ch.is_ascii_digit() || ch == '.',
            InputAreaType::MacAddress => ch.is_ascii_alphanumeric() || ch == ':',
        }
    }
}

/// 焦点标记
#[derive(Debug, Clone, Copy, Component)]
pub struct InputAreaFocused {
    pub last_active_time: SystemTime,
}
