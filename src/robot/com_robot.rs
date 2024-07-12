//! 来自机器人下位机的数据

use bevy_ecs::prelude::*;

pub const ADC_COUNT: usize = 5;
pub const IO_COUNT: usize = 8;
pub const MOTOR_COUNT: usize = 3;
pub const ROBOT_MOTOR_ROUND_POS_DELTA: i32 = 2500;

#[derive(Debug, Clone, Copy, Default, PartialEq, Resource)]
pub struct RobotLowerData {
    pub adc: [u16; ADC_COUNT],
    pub io: [bool; IO_COUNT],
    pub motor_status: [RobotMotorStatus; MOTOR_COUNT],
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Resource)]
pub struct RobotMotorStatus {
    /// 电机码盘位置
    /// 码盘是对电机转动角度进行计数的，这个数值变化2500表示电机转了一圈。
    /// 这个值是连续变化的，参考绝对编码器的原理。
    pub rotate_pos: i32,
    // 电机电流，单位：毫安
    pub current: u16,
}
