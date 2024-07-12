#![allow(unused)]

use bevy::prelude::*;

use super::{
    com_robot::RobotLowerData,
    motion::{RobotCtrl, RobotMotion},
    panorama_camera::PanoramaData,
};

/*
 * Part：插件
 */
pub(super) struct RobotMotionLogicPlugin;

impl Plugin for RobotMotionLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, robot_motion_set_target_system)
            .add_systems(
                FixedUpdate,
                robot_motion_set_moving_system.after(robot_motion_set_target_system),
            )
            .add_systems(FixedPostUpdate, robot_motion_activate_system);
    }
}

/*
 * Part：系统
 */

/// 机器人运动指令：入口
fn robot_motion_set_target_system(mut commands: Commands) {
    // TODO
    let robot_motion = RobotCtrl::Defense {
        ball_pos: glam::Vec2::new(0.0, 0.0),
    };

    commands.insert_resource(robot_motion);
}

/// 机器人运动指令：已确定目标
fn robot_motion_set_moving_system(mut commands: Commands, ctrl: Option<Res<RobotCtrl>>) {
    let Some(_ctrl) = ctrl else {
        return;
    };
    commands.remove_resource::<RobotCtrl>();
    // 添加运动指令
    let robot_motion = RobotMotion::default();
    commands.insert_resource(robot_motion);
}

/// 机器人运动指令：执行
fn robot_motion_activate_system(mut commands: Commands, motion: Option<Res<RobotMotion>>) {
    let Some(motion) = motion else {
        return;
    };
    // TODO
    let _speeds = motion.get_motor_speeds();
    commands.remove_resource::<RobotMotion>();
}

/*
 * Part：类型
 */
