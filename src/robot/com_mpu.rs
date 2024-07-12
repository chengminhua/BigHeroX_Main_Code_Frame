pub mod mpu_data;

use std::{
    io::{self, prelude::*},
    path::PathBuf,
    sync::Mutex,
    time::{Duration, SystemTime},
};

use bevy::prelude::*;
use mio::{Events, Interest, Poll, Token};
use mio_serial::{SerialPortBuilderExt, SerialPortInfo, SerialPortType, SerialStream};
use serde::{Deserialize, Serialize};
use static_init::dynamic;

use crate::{robot::ROBOT_CONFIG_DIR, traits::FastAccessData, TimeFlag};

use self::mpu_data::{MPURawData, MPU_DATA_BYTES_LENGTH};

/*
* Part: Plugin
*/

pub(super) struct RobotMPUPlugin;

impl Plugin for RobotMPUPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MPUConfig::load_or_default())
            .add_systems(Startup, enumerate_com_system)
            .add_systems(FixedPreUpdate, connect_serial_port_system)
            .add_systems(FixedPreUpdate, read_serial_port_system)
            .add_systems(
                FixedPreUpdate,
                read_buffer_system.after(read_serial_port_system),
            )
            .add_event::<MPUConnectEvent>();
    }
}

/*
* Part: Config
*/

#[derive(Resource, Default, Serialize, Deserialize)]
struct MPUConfig {
    serial_name: String,
}

impl FastAccessData<'_> for MPUConfig {
    fn file_path() -> &'static str {
        #[dynamic]
        static FILE_PATH: PathBuf = ROBOT_CONFIG_DIR.join("mpu.toml");
        #[dynamic]
        static FILE_PATH_STRING: String = FILE_PATH.to_string_lossy().to_string();
        &FILE_PATH_STRING
    }
}

/*
* Part: Event
*/

#[derive(Event)]
pub(super) struct MPUConnectEvent {
    pub com: usize,
}

#[derive(Event)]
#[allow(unused)]
pub(super) struct MPUDisConnectEvent;

fn enumerate_com_system() {
    let Ok(available_ports) = mio_serial::available_ports() else {
        warn!("Failed to Enumerate COM Ports!");
        return;
    };
    info!("Available ports: {:?}", available_ports);
    for port in available_ports {
        let SerialPortInfo {
            port_name,
            port_type,
        } = port;
        let SerialPortType::UsbPort(usb_port_info) = port_type else {
            continue;
        };
        let Some(serial_name) = usb_port_info.serial_number else {
            continue;
        };
        println!("Port: {} Serial Name: {}", port_name, serial_name);
    }
}

#[derive(Resource)]
pub(super) struct MPUSerialModule {
    poll: Poll,
    events: Events,
    stream: Mutex<SerialStream>,
}

const SERIAL_TOKEN: Token = Token(0);

fn connect_serial_port_system(
    mut commands: Commands,
    mut connect_events: EventReader<MPUConnectEvent>,
) {
    // Read event
    let Some(connect_event) = connect_events.read().next() else {
        return;
    };

    // Create a poll instance.
    let poll = match Poll::new() {
        Ok(poll) => poll,
        Err(err) => {
            warn!("Failed to create pull using mio! {err:?}");
            return;
        }
    };

    // Create storage for events. Since we will only register a single serialport, a
    // capacity of 1 will do.
    let events = Events::with_capacity(1);

    // Create the serial port
    info!("Opening COM{}", connect_event.com);
    let path = format!("COM{}", connect_event.com);
    let mut rx = match mio_serial::new(path, 1_000_000)
        .baud_rate(1_000_000)
        .data_bits(mio_serial::DataBits::Eight)
        .stop_bits(mio_serial::StopBits::One)
        .parity(mio_serial::Parity::None)
        .open_native_async()
    {
        Ok(rx) => rx,
        Err(err) => {
            warn!("Failed to open serial device using mio! {err:?}");
            return;
        }
    };

    poll.registry()
        .register(&mut rx, SERIAL_TOKEN, Interest::READABLE)
        .unwrap();

    commands.insert_resource(MPUSerialModule {
        poll,
        events,
        stream: Mutex::new(rx),
    });
}

/*
* Part: Read
*/

#[derive(Event)]
struct MPUFetchBufferEvent {
    buf: [u8; 1024],
    count: usize,
}

fn read_serial_port_system(
    module: Option<ResMut<MPUSerialModule>>,
    mut fetch_event_writer: EventWriter<MPUFetchBufferEvent>,
) {
    let Some(mut module) = module else {
        return;
    };
    let MPUSerialModule {
        poll,
        events,
        stream: stream_mutex,
    } = module.as_mut();
    let stream = &mut *stream_mutex
        .get_mut()
        .expect("Failed to get lock on MPU mio!!!");
    let mut buf = [0u8; 1024];

    // Poll to check if we have events waiting for us.
    poll.poll(events, None)
        .expect("Failed to pull events on MPU mio!!!");

    // Process each event.
    'read_events: for event in events.iter() {
        let token: Token = event.token();
        // Validate the token we registered our socket with,
        // in this example it will only ever be one but we
        // make sure it's valid none the less.
        match token {
            SERIAL_TOKEN => 'read_this_event: loop {
                // In this loop we receive all packets queued for the socket.
                match stream.read(&mut buf) {
                    Ok(count) => {
                        fetch_event_writer.send(MPUFetchBufferEvent { buf, count });
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        break 'read_this_event;
                    }
                    Err(e) => {
                        println!("Quitting due to read error: {}", e);
                        break 'read_events;
                    }
                }
            },
            _ => {
                // This should never happen as we only registered our
                // `UdpSocket` using the `UDP_SOCKET` token, but if it ever
                // does we'll log it.
                // warn!("Got event for unexpected token: {:?}", event);
            }
        }
    }
}

fn read_buffer_system(
    mut commands: Commands,
    mut fetch_buffer_event: EventReader<MPUFetchBufferEvent>,
) {
    let Some(MPUFetchBufferEvent { buf, count }) = fetch_buffer_event.read().last() else {
        return;
    };
    let Some(data) = std::iter::repeat(0)
        .take(count / MPU_DATA_BYTES_LENGTH)
        .enumerate()
        .map(|(index, _)| {
            &buf[(index * MPU_DATA_BYTES_LENGTH)..((index + 1) * MPU_DATA_BYTES_LENGTH)]
        })
        .filter_map(|buf_part| MPURawData::from_raw_parts(buf_part.iter()))
        .last()
    else {
        warn!("MPU: Wrong data format! Buf: {:?}", buf);
        return;
    };
    commands.spawn((
        data,
        TimeFlag {
            spawn_time: SystemTime::now(),
            exist_duration: Duration::from_millis(500),
        },
    ));
}
