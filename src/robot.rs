pub mod com_mpu;
pub mod com_robot;
pub mod logic;
pub mod motion;
pub mod panorama_camera;
pub mod test_cpp;
pub mod test_network_legacy;
pub mod test_rust;
pub mod ui;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use static_init::dynamic;
use std::fs::create_dir_all;

use bevy::prelude::*;

use crate::{
    field::FieldData, test_network_transfer::TestNetworkTransferPlugin, traits::FastAccessData,
    CRATE_DIR,
};

use self::{test_cpp::TestCppInputPlugin, test_rust::TestRustInputPlugin};

pub struct RobotPlugin {
    pub role: RobotRole,
}

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut App) {
        app
            // 添加角色
            .insert_resource(self.role)
            // 读取配置文件
            .insert_resource(RobotConfig::load_or_default())
            // 添加界面组件
            .add_plugins(ui::RobotUiPlugin)
            // 添加下位机组件
            .add_plugins(com_mpu::RobotMPUPlugin)
            // 添加输入
            .add_plugins(TestRustInputPlugin)
            .add_plugins(TestCppInputPlugin)
            .add_plugins(TestNetworkTransferPlugin)
            // To be continued
        ;
    }
}

/// 球员模式列表，如进攻球员、守门员等。
/// 应在一开始被添加至Resource中。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Resource)]
pub enum RobotRole {
    /// 前锋
    Striker,
    /// 守门员
    GoalKeeper,
}

impl std::fmt::Display for RobotRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str = match self {
            RobotRole::Striker => "前锋",
            RobotRole::GoalKeeper => "守门员",
        };
        f.write_str(display_str)
    }
}

#[dynamic]
pub static ROBOT_CONFIG_DIR: PathBuf = {
    let config_dir = CRATE_DIR.join("robot_config");
    if !config_dir.is_dir() {
        create_dir_all(&config_dir).expect("Failed to create robot_config dir!");
    }
    config_dir
};

/// 机器人设置
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize, Resource)]
pub struct RobotConfig {
    pub field_data: FieldData,
}

impl FastAccessData<'_> for RobotConfig {
    fn file_path() -> &'static str {
        #[dynamic]
        static FILE_PATH: PathBuf = ROBOT_CONFIG_DIR.join("config.toml");
        #[dynamic]
        static FILE_PATH_STRING: String = FILE_PATH.to_string_lossy().to_string();
        &FILE_PATH_STRING
    }
}
