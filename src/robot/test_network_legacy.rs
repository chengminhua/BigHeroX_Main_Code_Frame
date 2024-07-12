#![allow(unused)]

use std::{
    net::{Ipv4Addr, SocketAddrV4, UdpSocket},
    sync::{Arc, Condvar, Mutex},
};

use bevy::prelude::*;
use static_init::dynamic;

use crate::data_legacy::{LegacyPackFromCoach, LegacyPackFromRobot};

const BIND_IP_ADDRESS: [u8; 4] = [10, 31, 3, 124];
const BIND_PORT: u16 = 20091;
const COACH_IP_ADDRESS: [u8; 4] = [10, 31, 1, 2];
const COACH_PORT_STARTUP: u16 = 20090;
const COACH_PORT_AGENT: u16 = 20090;

/*
 * Part: Plugin
 */

pub(super) struct RobotNetworkLegacyPlugin;

impl Plugin for RobotNetworkLegacyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, network_startup_system)
            .add_systems(Update, network_update_system);
    }
}

/*
 * Part: System
 */

fn network_startup_system() {
    std::thread::spawn(connect_thread);
}

fn network_update_system() {
    let receive_signal = Arc::clone(&RECEIVE_SIGNAL);
    // 设置变量
    let mut guard = receive_signal
        .0
        .lock()
        .expect("Main: Failed to get lock!!!");
    *guard = true;
    // 通知
    receive_signal.1.notify_one();
}

/*
 * Part: Thread
 */

#[dynamic]
static RECEIVE_DATA: Arc<Mutex<Option<LegacyPackFromCoach>>> = Arc::new(Mutex::new(None));
#[dynamic]
static RECEIVE_SIGNAL: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(false), Condvar::new()));

fn connect_thread() {
    let socket = UdpSocket::bind(SocketAddrV4::new(
        Ipv4Addr::from(BIND_IP_ADDRESS),
        BIND_PORT,
    ))
    .unwrap_or_else(|err| panic!("Failed to bind {BIND_IP_ADDRESS:?}:{BIND_PORT}: {err}"));
    std::thread::spawn(move || socket_thread(socket));
}

fn socket_thread(socket: UdpSocket) {
    const HEADER_LEN: usize = 3;
    let mut bytes_cache = [0; 1024 * 1024];
    let mut loaded_byte_count = 0;
    let receive_signal = Arc::clone(&RECEIVE_SIGNAL);
    let mut first_time = true;
    #[allow(clippy::field_reassign_with_default)]
    loop {
        /*
         * 等待信号
         */
        let mut guard = receive_signal
            .0
            .lock()
            .expect("Thread: Failed to get lock!!!");
        while !*guard {
            guard = receive_signal
                .1
                .wait(guard)
                .expect("Thread: Panic to wait!!!");
        }
        *guard = false;
        drop(guard);
        /*
         * 发送数据
         */
        let mut pack_robot = LegacyPackFromRobot::default();
        pack_robot.id = 1;
        pack_robot.pos.x = rand::random();
        let coach_port = if first_time {
            first_time = false;
            COACH_PORT_STARTUP
        } else {
            COACH_PORT_AGENT
        };
        socket
            .send_to(
                &pack_robot.to_bytes(),
                SocketAddrV4::new(Ipv4Addr::from(COACH_IP_ADDRESS), coach_port),
            )
            .unwrap_or_else(|err| {
                panic!("Thread: Failed to Send data to {COACH_IP_ADDRESS:?}:{coach_port}! {err}")
            });
        info!("Data sent to {COACH_IP_ADDRESS:?}:{coach_port}! {pack_robot:?}");
        /*
         * 接收数据
         */
        // 接收数据
        loaded_byte_count += socket
            .recv(&mut bytes_cache[loaded_byte_count..])
            .expect("Failed to receive data header bytes!");
        // 还没接收完包头？
        if loaded_byte_count < HEADER_LEN {
            continue;
        }
        info!("Receive Header: {loaded_byte_count}");
        // 获取包长度
        let pack_len = bytes_cache[HEADER_LEN - 1] as usize;
        info!("Receive Data: pack_len: {pack_len} loaded: {loaded_byte_count}");
        // 还没接收完包体？
        if loaded_byte_count < pack_len {
            continue;
        }
        // 解析数据
        let data_from_coach = LegacyPackFromCoach::try_from_bytes(&bytes_cache[..pack_len])
            .unwrap_or_else(|err| {
                panic!(
                    "Failed to parse pack from bytes: {:?} err: {}",
                    &bytes_cache[..pack_len],
                    err
                )
            });
        info!("{data_from_coach:?}");
        // 有数据了，最后清空缓冲区
        loaded_byte_count = 0;
    }
}
