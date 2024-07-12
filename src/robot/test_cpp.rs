use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use bevy::prelude::*;
use static_init::dynamic;

use crate::{traits::SimpleService, TimeFlag};

/*
 * Part: Plugin
 */

pub(super) struct TestCppInputPlugin;

impl Plugin for TestCppInputPlugin {
    fn build(&self, app: &mut App) {
        app
            // Test: Cpp Thread
            .insert_resource(TestCppInputModule::new(TestCppInputConfig {
                sleep_time_sec: 3,
            }))
            .add_systems(FixedPreUpdate, test_cpp_input_update_system);
    }
}

/*
 * Part: System
 */

pub(super) fn test_cpp_input_update_system(
    mut commands: Commands,
    input_module: Res<TestCppInputModule>,
) {
    let Some(new_data) = input_module.take() else {
        return;
    };
    commands.spawn(new_data).insert(TimeFlag {
        spawn_time: SystemTime::now(),
        exist_duration: Duration::from_secs(5),
    });
}

/*
 * Part: Extern
 */

pub type TestCppInputConfig = test_cpp_input_rs::TestThreadInputConfig;

#[derive(Debug, Clone, Copy, Component)]
pub struct TestCppInputData(test_cpp_input_rs::TestThreadInputData);

impl Deref for TestCppInputData {
    type Target = test_cpp_input_rs::TestThreadInputData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TestCppInputData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[dynamic]
static DATA: Arc<Mutex<Option<TestCppInputData>>> = Arc::new(Mutex::new(None));

#[no_mangle]
pub extern "C" fn cpp_test_thread_input_load_data(
    new_data: test_cpp_input_rs::TestThreadInputData,
) {
    let data = Arc::clone(&DATA);
    let Ok(mut guard) = data.lock() else {
        return;
    };
    *guard = Some(TestCppInputData(new_data));
}

/*
 * Part: Input Module
 */

#[derive(Debug, Resource)]
pub struct TestCppInputModule {
    // 实现trait部分
    loop_data: Arc<Mutex<Option<TestCppInputData>>>,
    hook_continue: Option<Arc<Mutex<bool>>>,
    config: Arc<Mutex<TestCppInputConfig>>,
}

impl TestCppInputModule {
    pub fn new(config: TestCppInputConfig) -> Self {
        Self {
            loop_data: Arc::clone(&DATA),
            config: Arc::new(Mutex::new(config)),
            hook_continue: None,
        }
    }

    pub fn take(&self) -> Option<TestCppInputData> {
        self.loop_data.lock().expect("").take()
    }
}

impl SimpleService for TestCppInputModule {
    fn start_service(&mut self) {
        if self.hook_continue.is_some() {
            return;
        }
        let hook_continue = Arc::new(Mutex::new(true));
        self.hook_continue = Some(hook_continue);
        let config = *self.config.lock().expect("");
        test_cpp_input_rs::add_callback_func(cpp_test_thread_input_load_data);
        test_cpp_input_rs::start_service(config);
    }

    fn stop_service(&mut self) {
        if let Some(hook_continue) = self.hook_continue.take() {
            let mut guard = hook_continue.lock().expect("");
            *guard = false;
        }
        test_cpp_input_rs::stop_service();
    }

    fn is_service_running(&self) -> bool {
        test_cpp_input_rs::is_service_running()
    }
}
