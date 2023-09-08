use config::{Config, ConfigError, Environment, File, Map, Source, Value};
use serde_derive::{Deserialize, Serialize};

pub trait Merge {
    fn merge(&mut self, another: Self);
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(unused)]
pub struct Mode {
    pub debug: bool
}

impl Default for Mode {
    fn default() -> Self {
        Self {
            debug: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(unused)]
pub struct Server {
    pub external_url: String,
    pub http_port: u16,
    pub grpc_port: u16,
    pub healthz_port: u16,
    pub metrics_port: u16,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            external_url: String::from("http://127.0.0.1"),
            http_port: 8081,
            grpc_port: 8082,
            healthz_port: 10025,
            metrics_port: 9199,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(unused)]
pub struct Connections {
    pub max_idle: Option<u16>,
    pub max_open: Option<u16>,
}

impl Default for Connections {
    fn default() -> Self {
        Self {
            max_idle: Some(30),
            max_open: Some(30),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(unused)]
pub struct MySql {
    pub host: String,
    pub port: u16,
    pub dating: String,
    pub user: String,
    #[serde(rename = "pass")]
    pub password: String,
    pub connections: Connections,
}

impl Default for MySql {
    fn default() -> Self {
        Self {
            host: String::from("127.0.0.1"),
            port: 3306,
            dating: String::from("default"),
            user: String::from("root"),
            password: String::from(""),
            connections: Connections::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(unused)]
pub struct Db {
    pub  mysql: MySql,
}

impl Default for Db {
    fn default() -> Self {
        Self {
            mysql: MySql::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
#[allow(unused)]
pub enum LogLevel {
    Debug,
    #[default]
    Info,
    Warn,
    Error,
    Trace,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(unused)]
pub struct App {
    pub level: LogLevel,
}

impl Default for App {
    fn default() -> Self {
        Self {
            level: LogLevel::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(unused)]
pub struct Log {
    pub app: App,
}

impl Default for Log {
    fn default() -> Self {
        Self {
            app: App::default(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(unused)]
pub struct WatchDog {
    pub period: String,
    pub limit: u16,
    pub lock_timeout: String,
}

impl Default for WatchDog {
    fn default() -> Self {
        Self {
            period: String::from("5s"),
            limit: 10,
            lock_timeout: String::from("4s"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(unused)]
pub struct Backgroud {
    pub watchdog: WatchDog,
}

impl Default for Backgroud {
    fn default() -> Self {
        Self {
            watchdog: WatchDog::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[allow(unused)]
pub struct Settings {
    pub mode: Mode,
    pub server: Server,
    pub db: Db,
    pub log: Log,
    pub background: Backgroud,
}


impl Source for Settings {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new(self.clone())
    }

    fn collect(&self) -> Result<Map<String, Value>, ConfigError> {
        let json = toml::to_string(self).expect("Failed to serialize Settings");

        Ok(toml::from_str(&json).expect("Failed to deserialize Settings"))
    }
}

impl Settings {
    pub fn new<S: AsRef<str>>(debug: bool, filename: S) -> Self {
        let config = Config::builder()
            .add_source(Settings::default())
            .add_source(File::with_name(filename.as_ref()).required(false))
            .add_source(Environment::with_prefix("CONF").separator("_"))
            .set_override("mode.debug", debug).expect("Failed to set override")
            .build()
            .expect("Failed to build configuration");

        config.try_deserialize().expect("Failed to deserialize Settings")
    }
}