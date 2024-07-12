pub mod coach;
pub mod data_legacy;
pub mod error;
pub mod field;
pub mod robot;
pub mod test_network_transfer;
pub mod traits;
pub mod ui_components;

use std::{
    path::PathBuf,
    time::{Duration, SystemTime},
};

use bevy::{
    input::mouse::MouseWheel,
    prelude::*,
    window::WindowResolution,
    winit::{UpdateMode, WinitSettings},
};
use bevy_mod_picking::prelude::*;
use static_init::dynamic;

use coach::{CoachMode, CoachPlugin};
use robot::{RobotPlugin, RobotRole};
use ui_components::UiComponentsPlugin;

/*
 * Part：插件
 */

#[derive(Debug)]
pub struct MainPlugin {
    pub mode: Mode,
}

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        // 主窗口
        app
            // 设置固定时间系统的执行间隔，FPS 500
            .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(2)))
            // 设置运行更新模式：一直更新
            .insert_resource(WinitSettings {
                focused_mode: UpdateMode::Continuous,
                unfocused_mode: UpdateMode::Continuous,
            })
            // 设置窗口
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // 窗口标题
                    title: format!("BigHeroX RoboCup：{}", self.mode),
                    // 设置窗口大小，且无视系统DPI
                    resolution:
                        WindowResolution::new(1760.0, 990.0).with_scale_factor_override(1.0),
                    ..Default::default()
                }),
                ..low_latency_window_plugin()
            }));
        // 设置bevy_mod_picking相关参数``
        app
            .add_plugins(DefaultPickingPlugins.build().disable::<DefaultHighlightingPlugin>())
            // .insert_resource(DebugPickingMode::Normal)
            ;
        // 初始化图形
        app.add_systems(Startup, camera_setup_system);
        // 添加时间戳事件
        app.add_systems(FixedPreUpdate, time_flag_activate_system)
            // 鼠标滚轮事件
            // .add_systems(Update, scroll_system)
            // UI组件
            .add_plugins(UiComponentsPlugin);
        // 根据模式添加对应组件
        match self.mode {
            Mode::Coach { mode } => {
                app.add_plugins(CoachPlugin { mode });
            }
            Mode::Robot { role } => {
                app.add_plugins(RobotPlugin { role });
            }
        }
        // 添加场地绘制组件
        app.add_plugins(field::FieldPlugin);
    }
}

/*
 * Part：模式
 */

/// 程序运行模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    /// 教练机
    Coach { mode: CoachMode },
    /// 球员机
    Robot { role: RobotRole },
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Coach { mode } => {
                f.write_str("教练机：")?;
                f.write_str(&mode.to_string())
            }
            Mode::Robot { role } => {
                f.write_str("球员机：")?;
                f.write_str(&role.to_string())
            }
        }
    }
}

/// 初始化相机
fn camera_setup_system(mut commands: Commands) {
    // ui camera
    commands.insert_resource(ClearColor(Color::Rgba {
        red: 0.7,
        green: 0.7,
        blue: 0.7,
        alpha: 1.0,
    }));
    commands.spawn(Camera2dBundle {
        transform: {
            let mut tran = Transform::IDENTITY;
            tran.scale *= 0.0175;
            tran
        },
        ..Default::default()
    });
}

/// 鼠标滚轮事件
#[allow(unused)]
fn scroll_system(
    mut mouse_wheel_input_events: EventReader<MouseWheel>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    mouse_wheel_input_events.read().for_each(|event| {
        camera_query.iter_mut().for_each(|mut camera_tran| {
            camera_tran.scale *= 1.0 - (0.1 * event.y);
        });
    });
}

/*
 * Part：时间戳
 */

#[derive(Debug, Clone, Copy, Component)]
pub struct TimeFlag {
    pub spawn_time: SystemTime,
    pub exist_duration: Duration,
}

fn time_flag_activate_system(mut commands: Commands, quary_flags: Query<(Entity, &TimeFlag)>) {
    let now_time = SystemTime::now();
    quary_flags
        .iter()
        .filter(|(_, time_flag)| {
            now_time.duration_since(time_flag.spawn_time).expect("") >= time_flag.exist_duration
        })
        .for_each(|(entity, _)| {
            commands.get_or_spawn(entity).despawn_recursive();
        });
}

/*
 * 静态常量
 */

/// crate目录
#[dynamic]
static CRATE_DIR: PathBuf = {
    let current_dir = std::env::current_dir().expect("Failed to get current dir!");
    [
        current_dir.clone(),
        current_dir.join("bigherox-robocup"),
        current_dir.join(".."),
        current_dir.join("..").join("bigherox-robocup"),
        current_dir.join("..").join(".."),
        current_dir.join("..").join("..").join("bigherox-robocup"),
        current_dir.join("..").join("..").join(".."),
        current_dir
            .join("..")
            .join("..")
            .join("..")
            .join("bigherox-robocup"),
    ]
    .into_iter()
    .chain(std::env::split_paths(
        std::env::var_os("Path")
            .expect("Failed to find system path!")
            .as_os_str(),
    ))
    // 寻找Cargo.lock
    .find(|dir| dir.join("Cargo.lock").is_file())
    .expect("Unable to find workspace dir. Please change current dir to the workspace.")
};

#[dynamic]
static FONT_PATH: PathBuf = CRATE_DIR.join("fonts").join("SourceHanSansCN-Regular.otf");
