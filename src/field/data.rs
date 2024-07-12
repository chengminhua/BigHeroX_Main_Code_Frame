use serde::{Deserialize, Serialize};

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

/// 单位：米
/// 坐标系：场地中心为零点，敌方球门方向为x轴正方向，正前方朝着敌方球门时，左侧为y轴正方向，单位：米
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Resource)]
pub struct FieldData {
    /// 球场尺寸。
    pub field_size: Vec2,
    /// 大禁区尺寸。
    pub penalty_area_size: Vec2,
    /// 小禁区尺寸。
    pub goal_area_size: Vec2,
    /// 球门尺寸。
    pub gate_size: Vec3,
    /// 球场中心圆半径。
    pub center_circle_radius: f32,
    /// 球场四角1/4圆弧半径。
    pub corner_circle_radius: f32,
}

impl Default for FieldData {
    fn default() -> Self {
        Self {
            field_size: Vec2::new(18.0, 12.0),
            penalty_area_size: Vec2::new(5.0, 9.0),
            goal_area_size: Vec2::new(2.0, 7.0),
            gate_size: Vec3::new(1.5, 4.5, 2.0),
            center_circle_radius: 2.5,
            corner_circle_radius: 0.8,
        }
    }
}
