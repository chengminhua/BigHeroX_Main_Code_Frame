use std::f32::consts::PI;

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

/// 坐标系：场地中心为零点，敌方球门方向为x轴正方向，正前方朝着敌方球门时，左侧为y轴正方向，单位：米
#[derive(Debug, Clone, Copy, Default, Resource)]
pub struct RobotMotion {
    /// 坐标系：场地中心为零点，敌方球门方向为x轴正方向，正前方朝着敌方球门时，左侧为y轴正方向，单位：米
    pub now_pos: Vec2,
    /// 坐标系：场地中心为零点，敌方球门方向为x轴正方向，正前方朝着敌方球门时，左侧为y轴正方向，单位：米
    pub target_pos: Vec2,
    /// 单位：弧度（东侧为0，旋转一周为2*PI，增加方向为逆时针）
    pub speed_angle: f32,
    /// 单位：米每秒
    pub speed_mps: f32,
    /// 单位：转每分钟
    pub ball_take_wheel_speeds_rpm: Vec2,
    /// 吸球器触发指令
    pub ball_shot_prepare_ms: Option<u16>,
}

impl RobotMotion {
    /// 底盘轮子半径
    /// 单位：米
    pub const BUTTOM_WHEEL_RADIUS: f32 = 0.005;
    /// 底盘轮子安装位置相对于机器人正前方的角度
    /// 轮子顺序：后侧、左侧、右侧
    pub const WHEEL_INSTALL_PLACE_ANGLES: [f32; 3] = [-PI / 2.0, PI / 6.0, PI * 5.0 / 6.0];
    /// 底盘轮子的顺时针转动（相对于马达）相对于机器人正前方的角度
    /// 轮子顺序：后侧、左侧、右侧
    pub const WHEEL_ROLL_ANGLES: [f32; 3] = [-PI, PI / 3.0, -PI / 3.0];
    /// 轮子顺序：后侧、左侧、右侧
    /// 正值为顺时针
    /// 单位：转每分钟
    pub fn get_motor_speeds(&self) -> Vec3 {
        // 这两个相减顺序不影响cos函数结果
        Vec3::from(
            Self::WHEEL_ROLL_ANGLES
                .map(|roll_angle| self.speed_mps * f32::cos(self.speed_angle - roll_angle))
                .map(|roll_mps| roll_mps * 60.0)
                .map(|roll_mpm| roll_mpm / (2.0 * PI * Self::BUTTOM_WHEEL_RADIUS)),
        )
    }
}

/// 机器人的控制指令
/// 坐标系：场地中心为零点，敌方球门方向为x轴正方向，正前方朝着敌方球门时，左侧为y轴正方向，单位：米
#[derive(Debug, Clone, Copy, PartialEq, Resource)]
pub enum RobotCtrl {
    Pass {
        target_pos: Vec2,
    },
    /// 接球、抢球
    Catch {
        /// 球的位置
        ball_pos: Vec2,
    },
    /// 防御
    /// 此行为一般用于阻挡敌方球员射出的球
    /// 一般用于守门员，当然也可以用于进攻球员
    Defense {
        ball_pos: Vec2,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wheel_speeds() {
        let robot_motion = RobotMotion {
            speed_angle: 0.0,
            speed_mps: (2.0 * PI * RobotMotion::BUTTOM_WHEEL_RADIUS) / 60.0,
            ..Default::default()
        };
        let (back_rpm, left_rpm, right_rpm) =
            <(f32, f32, f32)>::from(robot_motion.get_motor_speeds());
        assert!(back_rpm - (-1.0) <= f32::EPSILON);
        assert!(left_rpm - (0.5) <= f32::EPSILON);
        assert!(right_rpm - (0.5) <= f32::EPSILON);
    }
}
