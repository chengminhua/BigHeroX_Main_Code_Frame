use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{traits::SimpleService, TimeFlag};

/*
 * Part: Plugin
 */

pub(super) struct TestRustInputPlugin;

impl Plugin for TestRustInputPlugin {
    fn build(&self, app: &mut App) {
        app
            // Test
            .insert_resource(TestRustInputModule::new(TestRustInputConfig {
                sleep_time: Duration::from_millis(1500),
            }))
            .add_systems(FixedPreUpdate, test_rust_input_update_system);
    }
}

/*
 * System
 */
fn test_rust_input_update_system(mut commands: Commands, input_module: Res<TestRustInputModule>) {
    let Some(data) = input_module.take() else {
        return;
    };
    commands.spawn(data).insert(TimeFlag {
        spawn_time: SystemTime::now(),
        exist_duration: Duration::from_secs(5),
    });
}

/*
 * Types
 */

#[derive(Debug, Clone, Copy, Default, Component)]
pub struct TestRustInputData {
    pub rand_num: i32,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct TestRustInputConfig {
    pub sleep_time: Duration,
}

#[derive(Debug, Resource)]
pub struct TestRustInputModule {
    // 实现trait部分
    loop_data: Arc<Mutex<Option<TestRustInputData>>>,
    hook_continue: Option<Arc<Mutex<bool>>>,
    config: Arc<Mutex<TestRustInputConfig>>,
}
/*
 * Part：按钮命令
 */

impl TestRustInputModule {
    pub fn new(config: TestRustInputConfig) -> Self {
        Self {
            loop_data: Arc::new(Mutex::new(None)),
            config: Arc::new(Mutex::new(config)),
            hook_continue: None,
        }
    }

    pub fn take(&self) -> Option<TestRustInputData> {
        self.loop_data.lock().expect("").take()
    }
}

fn input_thread(
    loop_data: Arc<Mutex<Option<TestRustInputData>>>,
    hook_continue: Arc<Mutex<bool>>,
    config: Arc<Mutex<TestRustInputConfig>>,
) {
    let mut first_time = true;
    while *hook_continue.lock().expect("") {
        // 获取当前配置
        let config_guard = config.lock().expect("");
        let config_now = *config_guard;
        drop(config_guard);
        // 休眠
        if first_time {
            first_time = false;
        } else {
            std::thread::sleep(config_now.sleep_time);
        }
        // 获取数据
        let data = TestRustInputData {
            rand_num: rand::random::<i32>(),
        };
        // 存储数据
        let mut loop_data_guard = loop_data.lock().expect("");
        *loop_data_guard = Some(data);
        drop(loop_data_guard);
    }
}

impl SimpleService for TestRustInputModule {
    fn start_service(&mut self) {
        if self.hook_continue.is_some() {
            return;
        }
        let hook_continue = Arc::new(Mutex::new(true));
        let hook_continue_outer = Arc::clone(&hook_continue);
        let loop_data = Arc::clone(&self.loop_data);
        let config = Arc::clone(&self.config);
        std::thread::spawn(move || input_thread(loop_data, hook_continue, config));
        self.hook_continue = Some(hook_continue_outer);
    }

    fn stop_service(&mut self) {
        if let Some(hook_continue) = self.hook_continue.take() {
            let mut guard = hook_continue.lock().expect("");
            *guard = false;
        }
    }

    fn is_service_running(&self) -> bool {
        self.hook_continue.is_some()
    }
}
