use glam::I16Vec2;

use num_enum_derive::{FromPrimitive, IntoPrimitive};
use primitive_byte_iter::ByteIter;

/// 消息类型
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
)]
#[repr(u8)]
pub enum LegacyMsgType {
    Info = 1,
    Cmd,
    Obst,
    Teammate,
    Kick,
    InitPos,
    /// 未定义
    #[default]
    UndefinedVal = 254,
}

/// 角色号
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
)]
#[repr(u8)]
pub enum LegacyCtrl {
    /// 不在线
    Offline = 255,
    /*
     * 一般控制指令
     */
    Stop = 0,
    Attack,
    Goalkeep,
    Defence,
    Pass,
    /// 传球：接球
    Catch,
    PassMove,
    CatchMove,
    CatchFocus,
    /// 手动控制
    Manual = 9,
    MoveTo = 11,
    Block,
    FocusOnBall,
    ProDef,
    SearchBall,
    ShiftAtk,
    RemoteCtrl,
    AroundBall,
    AtkCover,
    BackPos,
    LSAtk,
    LSAtkCover,
    ZoneDef = 30,
    /*
     * 2013年新增
     */
    Follow = 31,
    DefBall,
    DefGoal,
    Test = 40,
    Idle,
    /*
     * KickOff
     */
    KickOffPrimeReady = 50,
    KickOffSlaveReady,
    KickOffPrime,
    KickOffSlave,
    /*
     * FreeKick
     */
    FreeKickPrimeReady = 60,
    FreeKickSlaveReady,
    FreeKickPrime,
    FreeKickSlave,
    /*
     * GoalKick
     */
    GoalKickPrimeReady = 70,
    GoalKickSlaveReady,
    GoalKickPrime,
    GoalKickSlave,
    /*
     * ThrowIn
     */
    ThrowInPrimeReady = 80,
    ThrowInSlaveReady,
    ThrowInPrime,
    ThrowInSlave,
    /*
     * CornerKick
     */
    CornerKickPrimeReady = 90,
    CornerKickSlaveReady,
    CornerKickPrime,
    CornerKickSlave,
    /*
     * AntiKickOff
     */
    AntiKickOff = 95,
    /*
     * CornerKick
     */
    PenaltyReady = 100,
    Penalty,
    /*
     * Status
     */
    RobstNew = 110,
    RobstErr,
    RobstProbe,
    /*
     * 技术挑战赛
     */
    TechCompFindBall = 200,
    /// 未定义
    #[default]
    UndefinedVal = 0b0111_1111,
}

/// 策略号
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
)]
#[repr(u8)]
pub enum Match {
    Off = 0,
    Stop,
    Playing,
    ParkIn,
    ParkOut,
    Start,

    DroppedballReady = 9,
    DroppedballStart,
    /*
     * 状态：己方球权
     */
    KickOffReady = 11,
    KickOffStart,
    FreeKickReady,
    FreeKickStart,
    GoalKickReady,
    GoalKickStart,
    ThrowInReady,
    ThrowInStart,
    CornerKickReady,
    CornerKickStart,
    PenaltyReady,
    PenaltyStart,
    /*
     * 状态：对方球权
     */
    CounterKickoffReady = 31,
    CounterKickoffStart,
    CounterFreeKickReady,
    CounterFreeKickStart,
    CounterGoalKickReady,
    CounterGoalKickStart,
    CounterThrowInReady,
    CounterThrowInStart,
    CounterCornerKickReady,
    CounterCornerKickStart,
    CounterPenaltyReady,
    CounterPenaltyStart,
    /// 技术挑战赛
    TechChallange = 90,
    /*
     * 测试
     */
    TestPassing = 100,
    Test4pass,
    TestAllDef,
    TestMoveAround,
    TestCatchMove,
    /// 未定义
    #[default]
    UndefinedVal = 254,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LegacyPackBarrier {
    pub size: u8,
    pub pos: I16Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LegacyPackFromRobot {
    /*
     * 通用部分
     */
    pub id: u8,
    pub msg_type: LegacyMsgType,
    /// 当前位置
    pub pos: I16Vec2,
    /// 当前角度
    pub angle: i16,
    pub ctrl: LegacyCtrl,
    /// 是否已经持有球
    pub has_ball: bool,
    /// 发现的球
    pub found_ball: bool,
    pub found_ball_pos: I16Vec2,
    /// 运动状态
    pub velocity: u16,
    pub velocity_angle: i16,
    /// 传球
    pub pass_kick: bool,
    pub pass_target_pos: I16Vec2,
    /// 发现的障碍
    pub barriers: [LegacyPackBarrier; 10],
    /*
     * 电脑数据
     */
    pub computer_ac: u8,
    pub computer_battery_flag: u8,
    pub computer_battery_percent: u8,
    pub computer_working_second_count: u16,
    pub computer_cpu_percent: u8,
    pub computer_cpu_frequency_mhz: f32,
    /*
     * 软件数据
     */
    pub soft_version: f32,
    /*
     * 下位机（机器人）数据
     */
    pub robot_power_volt: u8,
    pub robot_charge: bool,
    /*
     * 传输
     */
    pub video_fps: u8,
    pub multicast_fps: u8,
}

const PACK_FROM_ROBOT_BYTE_LENGTH: usize = 176;
impl LegacyPackFromRobot {
    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let mut data = Self::default();
        let mut iter = ByteIter::new(bytes.iter());
        // 读取包头
        iter.next_u16();
        // 长度
        let set_len = iter
            .next_u8()
            .ok_or("Failed to get val from bytes: set_len")?;
        if bytes.len() < set_len as usize {
            return Err("Pack bytes len is lower than set len!!!".to_string());
        }
        // id
        data.id = iter.next_u8().ok_or("Failed to get val from bytes: id")?;
        // msg_type
        data.msg_type = LegacyMsgType::from(
            iter.next_u8()
                .ok_or("Failed to get val from bytes: msg_type")?,
        );
        // 机器人信息
        data.pos = iter
            .next_i16vec2()
            .ok_or("Failed to get val from bytes: pos")?;
        data.angle = iter
            .next_i16()
            .ok_or("Failed to get val from bytes: angle")?;
        // 机器人状态 and 球标记
        let byte_ctrl = iter
            .next_u8()
            .ok_or("Failed to get val from bytes: ctrl & found_ball")?;
        data.ctrl = LegacyCtrl::from(byte_ctrl);
        if let LegacyCtrl::UndefinedVal = data.ctrl {
            data.has_ball = (byte_ctrl & 0b1000_0000) != 0;
            data.ctrl = LegacyCtrl::from(byte_ctrl - if data.has_ball { 0b1000_0000 } else { 0 });
        }
        data.found_ball_pos = iter
            .next_i16vec2()
            .ok_or("Failed to get val from bytes: found_ball_pos")?;
        // 机器人运动速度
        data.velocity = iter
            .next_u16()
            .ok_or("Failed to get val from bytes: velocity")?;
        data.velocity_angle = iter
            .next_i16()
            .ok_or("Failed to get val from bytes: velocity_angle")?;
        // 传球
        data.pass_kick = iter
            .next_bool()
            .ok_or("Failed to get val from bytes: pass_kick")?;
        data.pass_target_pos = iter
            .next_i16vec2()
            .ok_or("Failed to get val from bytes: pass_target_pos")?;
        // 障碍物
        for barrier in data.barriers.iter_mut() {
            barrier.size = iter
                .next_u8()
                .ok_or("Failed to get val from bytes: barrier.size")?;
            barrier.pos = iter
                .next_i16vec2()
                .ok_or("Failed to get val from bytes: barrier.pos")?;
        }
        // 电脑信息
        data.computer_ac = iter
            .next_u8()
            .ok_or("Failed to get val from bytes: computer_ac")?;
        data.computer_battery_flag = iter
            .next_u8()
            .ok_or("Failed to get val from bytes: computer_battery_flag")?;
        data.computer_battery_percent = iter
            .next_u8()
            .ok_or("Failed to get val from bytes: computer_battery_percent")?;
        data.computer_working_second_count = iter
            .next_u16()
            .ok_or("Failed to get val from bytes: computer_working_second_count")?;
        data.computer_cpu_percent = iter
            .next_u8()
            .ok_or("Failed to get val from bytes: computer_cpu_percent")?;
        data.computer_cpu_frequency_mhz = iter
            .next_f32()
            .ok_or("Failed to get val from bytes: computer_cpu_frequency")?;
        data.soft_version = iter
            .next_f32()
            .ok_or("Failed to get val from bytes: soft_version")?;
        data.robot_power_volt = iter
            .next_u8()
            .ok_or("Failed to get val from bytes: robot_power_volt")?;
        data.robot_charge = iter
            .next_bool()
            .ok_or("Failed to get val from bytes: robot_charge")?;
        data.video_fps = iter
            .next_u8()
            .ok_or("Failed to get val from bytes: video_fps")?;
        data.multicast_fps = iter
            .next_u8()
            .ok_or("Failed to get val from bytes: multicast_fps")?;
        // 返回数据：记得校验
        let bytes_sum = bytes
            .iter()
            .map(ToOwned::to_owned)
            .fold(0u8, u8::wrapping_add);
        let set_sum = *bytes.last().expect("!!!");
        let bytes_sum = bytes_sum.wrapping_sub(set_sum);
        if bytes_sum != set_sum {
            return Err(format!(
                "Check failed: Set: {} Actual: {}",
                set_sum, bytes_sum
            ));
        }
        Ok(data)
    }

    pub fn to_bytes(&self) -> [u8; PACK_FROM_ROBOT_BYTE_LENGTH] {
        let mut bytes = [0; PACK_FROM_ROBOT_BYTE_LENGTH];
        let mut write_index = 0;
        let mut write_to_bytes = |src: &[u8]| {
            src.iter().for_each(|val| {
                bytes[write_index] = *val;
                write_index += 1;
            })
        };
        // 包头
        write_to_bytes(&[0x55, 0xAA]);
        // 长度
        write_to_bytes(&[PACK_FROM_ROBOT_BYTE_LENGTH as u8]);
        // 机器人信息
        write_to_bytes(&self.id.to_be_bytes());
        write_to_bytes(&u8::from(self.msg_type).to_be_bytes());
        // 机器人位置
        write_to_bytes(&self.pos.x.to_be_bytes());
        write_to_bytes(&self.pos.y.to_be_bytes());
        write_to_bytes(&self.angle.to_be_bytes());
        // 控制 and 是否有球？
        let ctrl_byte = u8::from(self.ctrl) | (if self.has_ball { 0b1000_0000 } else { 0 });
        write_to_bytes(&ctrl_byte.to_be_bytes());
        // 球坐标
        write_to_bytes(&self.found_ball_pos.x.to_be_bytes());
        write_to_bytes(&self.found_ball_pos.y.to_be_bytes());
        // 速度
        write_to_bytes(&self.velocity.to_be_bytes());
        write_to_bytes(&self.velocity_angle.to_be_bytes());
        // 传球
        write_to_bytes(&u8::from(self.pass_kick).to_be_bytes());
        write_to_bytes(&self.pass_target_pos.x.to_be_bytes());
        write_to_bytes(&self.pass_target_pos.y.to_be_bytes());
        // 障碍物
        self.barriers.into_iter().for_each(|barrier| {
            write_to_bytes(&barrier.size.to_be_bytes());
            write_to_bytes(&barrier.pos.x.to_be_bytes());
            write_to_bytes(&barrier.pos.y.to_be_bytes());
        });
        // 电脑信息
        write_to_bytes(&self.computer_ac.to_be_bytes());
        write_to_bytes(&self.computer_battery_flag.to_be_bytes());
        write_to_bytes(&self.computer_battery_percent.to_be_bytes());
        write_to_bytes(&self.computer_working_second_count.to_be_bytes());
        write_to_bytes(&self.computer_cpu_percent.to_be_bytes());
        write_to_bytes(&self.computer_cpu_frequency_mhz.to_be_bytes());
        // 软件信息
        write_to_bytes(&self.soft_version.to_be_bytes());
        // 机器人信息
        write_to_bytes(&self.robot_power_volt.to_be_bytes());
        write_to_bytes(&u8::from(self.robot_charge).to_be_bytes());
        // 组播信息
        write_to_bytes(&self.video_fps.to_be_bytes());
        write_to_bytes(&self.multicast_fps.to_be_bytes());
        // 校验和
        let sum = bytes.into_iter().fold(0u8, u8::wrapping_add);
        if let Some(last) = bytes.last_mut() {
            *last = sum;
        }
        // 返回数据
        bytes
    }
}

/*
 * Trait实现部分
 */

impl Default for LegacyPackFromRobot {
    fn default() -> Self {
        Self {
            id: 1,
            msg_type: LegacyMsgType::Teammate,
            pos: I16Vec2::new(800, 500),
            angle: 11451,
            ctrl: LegacyCtrl::Offline,
            has_ball: false,
            found_ball: true,
            found_ball_pos: I16Vec2::new(1919, 810),
            velocity: 0,
            velocity_angle: 0,
            pass_kick: true,
            pass_target_pos: I16Vec2::new(114, 514),
            barriers: [Default::default(); 10],
            computer_ac: 11,
            computer_battery_flag: 45,
            computer_battery_percent: 14,
            computer_working_second_count: 19198,
            computer_cpu_percent: 10,
            computer_cpu_frequency_mhz: 1145.14,
            soft_version: 0.114514,
            robot_power_volt: 0,
            robot_charge: true,
            video_fps: 114,
            multicast_fps: 51,
        }
    }
}

impl From<LegacyPackFromRobot> for [u8; PACK_FROM_ROBOT_BYTE_LENGTH] {
    fn from(val: LegacyPackFromRobot) -> Self {
        val.to_bytes()
    }
}

impl TryFrom<[u8; PACK_FROM_ROBOT_BYTE_LENGTH]> for LegacyPackFromRobot {
    type Error = String;

    fn try_from(value: [u8; PACK_FROM_ROBOT_BYTE_LENGTH]) -> Result<Self, Self::Error> {
        Self::try_from_bytes(&value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegacyPackFromCoach {
    /*
     * 通用部分
     */
    pub id: u8,
    pub msg_type: LegacyMsgType,
    pub players: [LegacyPackFromCoachPlayer; 5],
    pub barriers: [LegacyPackBarrier; 10],
    pub setup_pos: I16Vec2,
    pub found_ball: bool,
    pub ball_pos_from_coach: I16Vec2,
    /*
     * 额外部分
     */
    pub ctrl: LegacyCtrl,
    /*
     * 额外部分：MoveTo
     */
    pub target_pos: I16Vec2,
    pub target_angle: i16,
    pub speed: u8,
    /*
     * 额外部分：Def
     */
    pub def_angle: i16,
    pub def_dist: i16,
    /*
     * 额外部分：Pass
     */
    pub pass_target_pos: I16Vec2,
    /*
     * 额外部分：Catch
     */
    pub catch_from_pos: I16Vec2,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LegacyPackFromCoachPlayer {
    pub ctrl: LegacyCtrl,
    pub has_ball: bool,
    pub pos: I16Vec2,
    pub angle: i16,
}

/*
 * 方法部分
 */
const PACK_FROM_COACH_BYTE_LENGTH: usize = 205;
impl LegacyPackFromCoach {
    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let mut data = Self::default();
        let mut iter = ByteIter::new(bytes.iter());
        // 读取包头
        iter.next_u16();
        // 长度
        let set_len = iter
            .next_u8()
            .ok_or("Failed to get val from bytes: set_len")?;
        if bytes.len() < set_len as usize {
            return Err("Pack bytes len is lower than set len!!!".to_string());
        }
        // id
        data.id = iter.next_u8().ok_or("Failed to get val from bytes: id")?;
        // msg_type
        data.msg_type = LegacyMsgType::from(
            iter.next_u8()
                .ok_or("Failed to get val from bytes: msg_type")?,
        );
        // 读取数据：Info：球员
        for player in data.players.iter_mut() {
            let ctrl_byte = iter
                .next_u8()
                .ok_or("Failed to get val from bytes: player.ctrl_byte")?;
            player.ctrl = LegacyCtrl::from(ctrl_byte);
            player.has_ball = false;
            if let LegacyCtrl::UndefinedVal = player.ctrl {
                player.has_ball = (ctrl_byte & 0b1000_0000) != 0;
                player.ctrl = LegacyCtrl::from(if player.has_ball {
                    ctrl_byte - 0b1000_0000
                } else {
                    ctrl_byte
                });
            }
            player.pos = iter
                .next_i16vec2()
                .ok_or("Failed to get val from bytes: player.pos")?;
            player.angle = iter
                .next_i16()
                .ok_or("Failed to get val from bytes: player.angle")?;
        }
        // 读取数据：Info：障碍
        for barrier in data.barriers.iter_mut() {
            barrier.size = iter
                .next_u8()
                .ok_or("Failed to get val from bytes: barrier.size")?;
            barrier.pos = iter
                .next_i16vec2()
                .ok_or("Failed to get val from bytes: barrier.pos")?;
        }
        // 读取数据：Info：结尾
        data.setup_pos = iter
            .next_i16vec2()
            .ok_or("Failed to get val from bytes: setup_pos")?;
        data.found_ball = iter
            .next_bool()
            .ok_or("Failed to get val from bytes: found_ball")?;
        data.ball_pos_from_coach = iter
            .next_i16vec2()
            .ok_or("Failed to get val from bytes: ball_pos_from_coach")?;
        // 读取数据：获取控制指令类型
        data.ctrl = LegacyCtrl::from(iter.next_u8().ok_or("Failed to get val from bytes: ctrl")?);
        // 读取数据：控制指令
        match data.ctrl {
            LegacyCtrl::MoveTo => {
                data.target_pos = iter
                    .next_i16vec2()
                    .ok_or("Failed to get val from bytes: target_pos")?;
                data.target_angle = iter
                    .next_i16()
                    .ok_or("Failed to get val from bytes: target_angle")?;
                data.speed = iter
                    .next_u8()
                    .ok_or("Failed to get val from bytes: speed")?;
            }
            LegacyCtrl::Defence => {
                data.def_angle = iter
                    .next_i16()
                    .ok_or("Failed to get val from bytes: def_angle")?;
                data.def_dist = iter
                    .next_i16()
                    .ok_or("Failed to get val from bytes: def_dist")?;
            }
            LegacyCtrl::Pass => {
                data.pass_target_pos = iter
                    .next_i16vec2()
                    .ok_or("Failed to get val from bytes: pass_target_pos")?;
            }
            LegacyCtrl::Catch => {
                data.catch_from_pos = iter
                    .next_i16vec2()
                    .ok_or("Failed to get val from bytes: catch_from_pos")?;
            }
            _ => (),
        }
        // 返回数据：记得校验
        let bytes_sum = bytes
            .iter()
            .map(ToOwned::to_owned)
            .fold(0u8, u8::wrapping_add);
        let set_sum = *bytes.last().expect("!!!");
        let bytes_sum = bytes_sum.wrapping_sub(set_sum);
        if bytes_sum != set_sum {
            return Err(format!(
                "Check failed: Set: {} Actual: {}",
                set_sum, bytes_sum
            ));
        }
        Ok(data)
    }

    pub fn to_bytes(&self) -> [u8; PACK_FROM_COACH_BYTE_LENGTH] {
        let mut bytes = [0u8; PACK_FROM_COACH_BYTE_LENGTH];
        let mut write_index = 0;
        let mut write_to_bytes = |src: &[u8]| {
            src.iter().for_each(|val| {
                bytes[write_index] = *val;
                write_index += 1;
            })
        };
        // 包头
        write_to_bytes(&[0x55, 0xAA]);
        // 长度
        write_to_bytes(&[PACK_FROM_COACH_BYTE_LENGTH as u8]);
        // 编号
        write_to_bytes(&self.id.to_be_bytes());
        // 消息类型
        write_to_bytes(&u8::from(self.msg_type).to_be_bytes());
        // 机器人
        self.players.into_iter().for_each(|player| {
            // 状态
            write_to_bytes(
                &(u8::from(player.ctrl) | if player.has_ball { 0b1000_0000 } else { 0 })
                    .to_be_bytes(),
            );
            // 位置
            write_to_bytes(&player.pos.x.to_be_bytes());
            write_to_bytes(&player.pos.y.to_be_bytes());
            // 角度
            write_to_bytes(&player.angle.to_be_bytes());
        });
        // 障碍物
        self.barriers.into_iter().for_each(|barrier| {
            write_to_bytes(&barrier.size.to_be_bytes());
            write_to_bytes(&barrier.pos.x.to_be_bytes());
            write_to_bytes(&barrier.pos.y.to_be_bytes());
        });
        // 初始位置
        write_to_bytes(&self.setup_pos.x.to_be_bytes());
        write_to_bytes(&self.setup_pos.y.to_be_bytes());
        write_to_bytes(&u8::from(self.found_ball).to_be_bytes());
        write_to_bytes(&self.ball_pos_from_coach.x.to_be_bytes());
        write_to_bytes(&self.ball_pos_from_coach.y.to_be_bytes());
        // 操作
        write_to_bytes(&u8::from(self.ctrl).to_be_bytes());
        match self.ctrl {
            LegacyCtrl::MoveTo => {
                // 位置
                write_to_bytes(&self.target_pos.x.to_be_bytes());
                write_to_bytes(&self.target_pos.y.to_be_bytes());
                // 角度
                write_to_bytes(&self.target_angle.to_be_bytes());
                write_to_bytes(&self.speed.to_be_bytes());
            }
            LegacyCtrl::Defence => {
                write_to_bytes(&self.def_dist.to_be_bytes());
                write_to_bytes(&self.def_angle.to_be_bytes());
            }
            LegacyCtrl::Pass => {
                // 位置
                write_to_bytes(&self.pass_target_pos.x.to_be_bytes());
                write_to_bytes(&self.pass_target_pos.y.to_be_bytes());
            }
            LegacyCtrl::Catch => {
                // 位置
                write_to_bytes(&self.catch_from_pos.x.to_be_bytes());
                write_to_bytes(&self.catch_from_pos.y.to_be_bytes());
            }
            _ => (),
        }
        // 校验和
        let sum = bytes.into_iter().fold(0u8, u8::wrapping_add);
        if let Some(last) = bytes.last_mut() {
            *last = sum;
        }
        // 返回数据
        bytes
    }
}

/*
 * Trait实现部分
 */

impl Default for LegacyPackFromCoach {
    fn default() -> Self {
        Self {
            id: 1,
            msg_type: LegacyMsgType::Cmd,
            players: [Default::default(); 5],
            barriers: [Default::default(); 10],
            setup_pos: I16Vec2::new(800, 900),
            found_ball: true,
            ball_pos_from_coach: I16Vec2::new(800, 500),
            ctrl: LegacyCtrl::Offline,
            target_pos: I16Vec2::new(1600, 400),
            target_angle: 0x7fff,
            speed: 9,
            def_angle: 0x3fff,
            def_dist: 100,
            pass_target_pos: I16Vec2::new(800, 200),
            catch_from_pos: I16Vec2::new(400, 500),
        }
    }
}

impl From<LegacyPackFromCoach> for [u8; PACK_FROM_COACH_BYTE_LENGTH] {
    fn from(val: LegacyPackFromCoach) -> Self {
        val.to_bytes()
    }
}

impl TryFrom<[u8; PACK_FROM_COACH_BYTE_LENGTH]> for LegacyPackFromCoach {
    type Error = String;

    fn try_from(value: [u8; PACK_FROM_COACH_BYTE_LENGTH]) -> Result<Self, Self::Error> {
        Self::try_from_bytes(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_convert_robot() {
        let original_data: LegacyPackFromRobot = Default::default();
        let convert_bytes = original_data.to_bytes();
        let new_data = LegacyPackFromRobot::try_from_bytes(&convert_bytes)
            .expect("Failed to read data from bytes!");
        assert_eq!(original_data, new_data);
    }

    #[test]
    fn round_convert_coach() {
        let original_data: LegacyPackFromCoach = Default::default();
        let convert_bytes = original_data.to_bytes();
        let new_data = LegacyPackFromCoach::try_from_bytes(&convert_bytes)
            .expect("Failed to read data from bytes!");
        assert_eq!(original_data, new_data);
    }
}
