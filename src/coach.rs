use bevy::prelude::*;

pub struct CoachPlugin {
    pub mode: CoachMode,
}

impl Plugin for CoachPlugin {
    fn build(&self, app: &mut App) {
        app
            // 添加模式
            .insert_resource(self.mode);
    }
}

/// 教练模式列表
/// 应在一开始被添加至Resource中。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Resource)]
pub enum CoachMode {
    Normal,
    SkillCompetition,
    ShotCompetition,
}

impl std::fmt::Display for CoachMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str = match self {
            CoachMode::Normal => "小组赛",
            CoachMode::SkillCompetition => "技巧挑战赛",
            CoachMode::ShotCompetition => "射门挑战赛",
        };
        f.write_str(display_str)
    }
}
