#![allow(unused)]

use bevy::prelude::*;
use static_init::dynamic;
use std::{
    io::prelude::*,
    net::TcpStream,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use std::net::TcpListener;

/*
 * 测试部分
 */

use serde::{Deserialize, Serialize};

use crate::TimeFlag;

pub const TEST_ADDRESS: &str = "127.0.0.1:11451";

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct TestSharedData {
    pub rand_data: i16,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct TestSharedResponse {
    pub status: u16,
}

pub(super) struct TestNetworkSendPlugin;

impl Plugin for TestNetworkSendPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, network_send_startup_system);
    }
}

pub(super) fn network_send_startup_system() {
    info!("Network send setup! addr: {}", TEST_ADDRESS);
    let _thread = std::thread::spawn(listen_thread);
}

fn listen_thread() {
    let Ok(tcp_listener) = TcpListener::bind(TEST_ADDRESS) else {
        panic!("Failed to bind {}", TEST_ADDRESS);
    };
    info!("Listening to {}", TEST_ADDRESS);
    for stream in tcp_listener.incoming() {
        let stream = stream.unwrap_or_else(|err| {
            panic!(
                "Failed to open stream from incoming {}. {err}",
                TEST_ADDRESS
            )
        });
        let peer_addr = stream.peer_addr().expect("Failed to get peer addr!");
        info!("New stream! addr: {}", peer_addr);
        std::thread::spawn(move || send_data_thread(stream));
    }
}

fn send_data_thread(mut stream: TcpStream) {
    let mut receive_data_str = String::new();
    let mut first_time = true;
    loop {
        // 并非首轮？则接收回复
        if !first_time {
            info!("Waiting for response...");
            // 接收回复
            let _read_len = stream
                .read_to_string(&mut receive_data_str)
                .unwrap_or_else(|err| panic!("Failed to read data from stream. {err}"));
            let Ok(_response) = toml::from_str::<TestSharedResponse>(&receive_data_str) else {
                warn!("{} is not response.", receive_data_str);
                continue;
            };
            // 清空缓冲区
            receive_data_str.clear();
        }
        first_time = false;
        // 发送数据
        let rand_num = rand::random();
        let send_data = TestSharedData {
            rand_data: rand_num,
            message: format!("Rand a number: {}", rand_num),
        };
        // info!("Sending data: {:?}", send_data);
        let send_str =
            toml::to_string(&send_data).expect("Failed to parse string from send object!");
        let send_bytes = send_str.as_bytes();
        stream
            .write_all(&[send_bytes.len() as u8])
            .expect("Failed to write data len to tcp stream in sys thread!!!");
        stream
            .write_all(send_bytes)
            .expect("Failed to write data to tcp stream in sys thread!!!");
        info!("Finish Sending data: {}", send_str);
    }
}

/*
 * Part: Plugin
 */

pub(super) struct TestNetworkTransferPlugin;

impl Plugin for TestNetworkTransferPlugin {
    fn build(&self, app: &mut App) {
        app
            // Test: Network
            .add_systems(Startup, test_network_receive_setup_system)
            .add_systems(FixedPreUpdate, test_network_receive_system)
            .add_systems(FixedPostUpdate, test_network_show_data_system);
    }
}

// #[dynamic]
// static TCP_STREAM: Arc<Mutex<Option<TcpStream>>> = Arc::new(Mutex::new(None));
#[dynamic]
static RECEIVE_DATA: Arc<Mutex<Option<TestSharedData>>> = Arc::new(Mutex::new(None));

fn receive_data_thread() {
    info!("Connecting: {TEST_ADDRESS}");
    let Ok(mut tcp_stream) = TcpStream::connect(TEST_ADDRESS) else {
        panic!("Failed to connect to {}", TEST_ADDRESS);
    };
    info!("Connected: {TEST_ADDRESS}");
    let mut receive_len = [0];
    let mut receive_bytes: Vec<u8> = Vec::new();
    let receive_data_ref = Arc::clone(&RECEIVE_DATA);
    loop {
        // 接收数据
        tcp_stream
            .read_exact(&mut receive_len)
            .unwrap_or_else(|_| panic!("Failed to read data len from {}", TEST_ADDRESS));
        receive_bytes.resize(receive_len[0] as usize, 0);
        tcp_stream
            .read_exact(&mut receive_bytes)
            .unwrap_or_else(|_| panic!("Failed to read data from {}", TEST_ADDRESS));
        info!("{TEST_ADDRESS}: data_len: {}", receive_bytes.len());
        let Ok(data_str) = String::from_utf8(Vec::clone(&receive_bytes)) else {
            continue;
        };
        let Ok(data) = toml::from_str::<TestSharedData>(&data_str) else {
            continue;
        };
        info!("{TEST_ADDRESS}: Read data! data: {:?}", data);
        let mut receive_data_guard = receive_data_ref
            .lock()
            .expect("Failed to get receive data lock in sys thread!!!");
        *receive_data_guard = Some(data);
        // 发送回复
        let response = TestSharedResponse { status: 200 };
        let response_str =
            toml::to_string(&response).expect("Failed to parse string from response object!");
        tcp_stream
            .write_all(response_str.as_bytes())
            .expect("Failed to write response in sys thread!!!");
        // 清空缓冲区
        receive_bytes.clear();
    }
}

pub(super) fn test_network_receive_setup_system() {
    let _thread = std::thread::spawn(receive_data_thread);
}

pub(super) fn test_network_receive_system(mut commands: Commands) {
    let new_data_ref = Arc::clone(&RECEIVE_DATA);
    let mut new_data_guard = new_data_ref
        .lock()
        .expect("Failed to get receive data lock in bevy system!!!");
    let Some(data) = new_data_guard.take() else {
        return;
    };
    commands.spawn(data).insert(TimeFlag {
        spawn_time: SystemTime::now(),
        exist_duration: Duration::from_secs(5),
    });
}

pub(super) fn test_network_show_data_system(data_query: Query<(&TestSharedData, &TimeFlag)>) {
    let mut data_list = data_query.into_iter().collect::<Vec<_>>();
    let now_time = SystemTime::now();
    data_list.sort_by_key(|(_, TimeFlag { spawn_time, .. })| {
        now_time
            .duration_since(*spawn_time)
            .expect("111")
            .as_micros()
    });
    let Some((data, _)) = data_list.first() else {
        return;
    };
    // 输出数据
    info!("Received: {}", data.message);
}
