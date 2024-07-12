//! 来自全景相机的数据

use bevy_ecs::prelude::*;
use glam::Vec2;

/// 记录数据：已设置入场点
/// 坐标系：场地中心为零点，敌方球门方向为x轴正方向，正前方朝着敌方球门时，左侧为y轴正方向，单位：米
#[derive(Debug, Clone, Copy, Default, Resource)]
pub struct PanoramaEntryData {
    /// 设定值
    pub set_entry_pos: Vec2,
    /// 设定值
    pub set_entry_angle: f32,
    /// 设定时本地记录
    pub entry_angle_z: f32,
}

/// 全景相机返回数据
/// 坐标系：场地中心为零点，敌方球门方向为x轴正方向，正前方朝着敌方球门时，左侧为y轴正方向，单位：米
#[derive(Debug, Clone, Default, Resource)]
pub struct PanoramaData {
    /// 当前位置
    pub pos: Vec2,
    /// 障碍物列表
    pub barriers: Vec<PanoramaBarrier>,
}

/// 全景相机扫描到的障碍物
/// 坐标系：场地中心为零点，敌方球门方向为x轴正方向，正前方朝着敌方球门时，左侧为y轴正方向，单位：米
/// TODO：改成区间而不是圆形，以提高后续协同的准确度
#[derive(Debug, Clone, Copy, Default, Resource)]
pub struct PanoramaBarrier {
    /// 障碍物位置
    pub pos: Vec2,
    /// 障碍物大小
    pub size: f32,
}

/// 全景相机返回图片
/// TODO
#[derive(Debug, Clone, Copy, Default, Resource)]
pub struct PanoramaImage {}
