use std::path::Path;

use serde::{de::DeserializeOwned, Serialize};

use std::{
    marker::Send,
    sync::{Arc, Condvar, Mutex},
};

/// 可快速存取的数据。可以一行代码完成读取和写入操作。
pub trait FastAccessData<'de>: Serialize + DeserializeOwned {
    /// 文件路径
    fn file_path() -> &'static str;

    /// 添加至文件开头
    fn file_header() -> &'static str {
        ""
    }

    /// 添加至文件结尾
    fn file_trailer() -> &'static str {
        ""
    }

    fn save(&self) -> Result<(), FastAccessDataError> {
        // Create Parent dir
        let parent_dir = Path::new(Self::file_path())
            .parent()
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "FastAccessData: Failed to get parent!!!",
            ))?;
        std::fs::create_dir_all(parent_dir)?;
        // Write config
        let mut toml_string = toml::to_string_pretty(self)?;
        toml_string.insert_str(0, Self::file_header());
        toml_string.push_str(Self::file_trailer());
        Ok(std::fs::write(Self::file_path(), toml_string)?)
    }

    fn load() -> Result<Self, FastAccessDataError> {
        let toml_string = std::fs::read_to_string(Self::file_path())?;
        Ok(toml::from_str(&toml_string)?)
    }

    fn load_or_default() -> Self
    where
        Self: Default,
    {
        Self::load().unwrap_or_else(|err| {
            println!("load_or_default: has error: {:?}, now using default.", err);
            let val = Self::default();
            // Save file
            Self::save(&val).unwrap_or_else(|_| {
                panic!(
                    "load_or_default: Failed to write default on {}",
                    Self::file_path()
                )
            });
            val
        })
    }
}

#[derive(Debug)]
pub enum FastAccessDataError {
    Io(std::io::Error),
    TomlDe(toml::de::Error),
    TomlSer(toml::ser::Error),
}

impl From<std::io::Error> for FastAccessDataError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<toml::de::Error> for FastAccessDataError {
    fn from(value: toml::de::Error) -> Self {
        Self::TomlDe(value)
    }
}

impl From<toml::ser::Error> for FastAccessDataError {
    fn from(value: toml::ser::Error) -> Self {
        Self::TomlSer(value)
    }
}

pub trait InputModule<T>: Service {
    fn loop_data(&self) -> &Arc<Mutex<Option<T>>>;
    fn loop_condvar(&self) -> &Arc<Condvar>;
    fn loop_mutex(&self) -> &Arc<Mutex<bool>>;
    fn start_service(&'static mut self)
    where
        Self: Send,
    {
        if !self.is_setup() {
            self.setup();
        }
        if self.is_service_running() {
            return;
        }
        self.set_service_running(true);
        let local_loop_condvar = Arc::clone(self.loop_condvar());
        let local_loop_mutex = Arc::clone(self.loop_mutex());
        std::thread::spawn(move || {
            while self.is_service_running() {
                self.fetch_data();
                let mut guard = local_loop_mutex.lock().unwrap();
                self.store_data();
                *guard = true;
                local_loop_condvar.notify_one();
            }
        });
    }
    fn fetch_data(&mut self);
    fn store_data(&mut self);
}

/// 用于自带一个循环线程的服务，如主循环、输入来源循环。
pub trait Service {
    fn is_service_running(&self) -> bool;
    fn set_service_running(&mut self, new_status: bool);
    fn is_setup(&self) -> bool;
    /// 初始化操作，如启动设备。
    fn setup(&mut self);
    fn start_service(&'static mut self)
    where
        Self: Send;
    fn stop_service(&mut self);
}

/// 用于自主实现的服务。
pub trait SimpleService {
    fn is_service_running(&self) -> bool;
    fn start_service(&mut self);
    fn stop_service(&mut self);
}
