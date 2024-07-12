use std::time::SystemTime;

use bevy::prelude::*;

use crate::{
    robot::{
        com_mpu::MPUConnectEvent,
        test_cpp::{TestCppInputData, TestCppInputModule},
        test_rust::{TestRustInputData, TestRustInputModule},
    },
    traits::SimpleService,
    TimeFlag,
};

/*
 * Part：插件
 */

pub(super) struct RobotUiFunctionPlugin;

impl Plugin for RobotUiFunctionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, activate_toggle_cpp_system)
            .add_systems(Update, show_value_cpp_system)
            .add_systems(Update, activate_toggle_rust_system)
            .add_systems(Update, show_value_rust_system)
            .add_systems(Update, activate_connect_mpu_system);
    }
}

/*
 * Part：输入测试
 */

#[derive(Debug, Clone, Copy, Component)]
pub(super) struct ToggleCppInputActivator;

fn activate_toggle_cpp_system(
    button_query: Query<&Interaction, (With<ToggleCppInputActivator>, Changed<Interaction>)>,
    mut cpp_input_module: ResMut<TestCppInputModule>,
) {
    let Some(_) = button_query
        .into_iter()
        .find(|interaction| matches!(interaction, Interaction::Pressed))
    else {
        return;
    };
    if cpp_input_module.is_service_running() {
        cpp_input_module.stop_service();
    } else {
        cpp_input_module.start_service();
    }
}

fn show_value_cpp_system(
    button_query: Query<&Children, With<ToggleCppInputActivator>>,
    query_cpp_data: Query<(&TestCppInputData, &TimeFlag)>,
    mut text_query: Query<&mut Text>,
) {
    for children in button_query.into_iter() {
        let mut data_list = query_cpp_data.into_iter().collect::<Vec<_>>();
        let now_time = SystemTime::now();
        data_list.sort_by_key(|(_, TimeFlag { spawn_time, .. })| {
            now_time
                .duration_since(*spawn_time)
                .expect("111")
                .as_micros()
        });
        let Some((data, _)) = data_list.first() else {
            return;
        };
        let mut text = text_query.get_mut(children[0]).unwrap();
        text.sections[0].value = format!("Cpp: {} {}", data.data_wide, data.data_float);
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub(super) struct ToggleRustInputActivator;

fn activate_toggle_rust_system(
    button_query: Query<&Interaction, (With<ToggleRustInputActivator>, Changed<Interaction>)>,
    mut rust_input_module: ResMut<TestRustInputModule>,
) {
    let Some(_) = button_query
        .into_iter()
        .find(|interaction| matches!(interaction, Interaction::Pressed))
    else {
        return;
    };
    if rust_input_module.is_service_running() {
        rust_input_module.stop_service();
    } else {
        rust_input_module.start_service();
    }
}

fn show_value_rust_system(
    button_query: Query<&Children, With<ToggleRustInputActivator>>,
    query_rust_data: Query<(&TestRustInputData, &TimeFlag)>,
    mut text_query: Query<&mut Text>,
) {
    for children in button_query.into_iter() {
        let mut data_list = query_rust_data.into_iter().collect::<Vec<_>>();
        let now_time = SystemTime::now();
        data_list.sort_by_key(|(_, TimeFlag { spawn_time, .. })| {
            now_time
                .duration_since(*spawn_time)
                .expect("111")
                .as_micros()
        });
        let Some((data, _)) = data_list.first() else {
            return;
        };
        let mut text = text_query.get_mut(children[0]).unwrap();
        text.sections[0].value = format!("rust: {}", data.rand_num);
    }
}

/*
 * Part：教练机
 */

#[derive(Debug, Clone, Copy, Component)]
pub(super) struct ConnectCoachInputArea;

#[derive(Debug, Clone, Copy, Component)]
pub(super) struct ConnectCoachActivator;

/*
 * Part：全景相机
 */

#[allow(unused)]
#[derive(Debug, Clone, Copy, Component)]
pub(super) struct ConnectPanoramaCameraActivator;

/*
 * Part：下位机
 */

#[allow(unused)]
#[derive(Debug, Clone, Copy, Component)]
pub(super) struct ConnectRobotActivator;

/*
 * Part：MPU
 */

#[derive(Debug, Clone, Copy, Component)]
pub(super) struct ConnectMPUInputArea;

#[derive(Debug, Clone, Copy, Component)]
pub(super) struct ConnectMPUActivator;

fn activate_connect_mpu_system(
    button_query: Query<&Interaction, (With<ConnectMPUActivator>, Changed<Interaction>)>,
    input_area_query: Query<&Children, With<ConnectMPUInputArea>>,
    text_query: Query<&Text>,
    mut connect_mpu_plugin_event: EventWriter<MPUConnectEvent>,
) {
    let Some(_) = button_query
        .into_iter()
        .find(|interaction| matches!(interaction, Interaction::Pressed))
    else {
        return;
    };
    let Some(children) = input_area_query.into_iter().next() else {
        return;
    };
    for child in children {
        let Ok(text_sections) = text_query.get(*child) else {
            continue;
        };
        let text = &text_sections.sections[0].value;
        let Ok(com_id) = text.parse::<u32>() else {
            continue;
        };
        connect_mpu_plugin_event.send(MPUConnectEvent {
            com: com_id as usize,
        });
        break;
    }
}
