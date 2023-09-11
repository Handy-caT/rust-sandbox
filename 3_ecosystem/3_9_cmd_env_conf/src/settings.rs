use std::net::{IpAddr};
use std::time::Duration;
use config::{Config, ConfigError, Environment, File, Map, Source, Value};
use serde_derive::{Deserialize, Serialize};
use smart_default::SmartDefault;
use url::Url;




#[derive(Debug, Serialize, Deserialize, Clone)]
#[derive(SmartDefault)]
#[allow(unused)]
pub struct Mode {
    #[default(false)]
    pub debug: bool
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[derive(SmartDefault)]
#[allow(unused)]
pub struct Server {
    #[default("http:////127.0.0.1".parse().unwrap())]
    pub external_url: Url,
    #[default(8081)]
    pub http_port: u16,
    #[default(8082)]
    pub grpc_port: u16,
    #[default(10025)]
    pub healthz_port: u16,
    #[default(9199)]
    pub metrics_port: u16,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[derive(SmartDefault)]
#[allow(unused)]
pub struct Connections {
    #[default(30)]
    pub max_idle: u16,
    #[default(30)]
    pub max_open: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[derive(SmartDefault)]
#[allow(unused)]
pub struct MySql {
    #[default("127.0.0.1".parse().unwrap())]
    pub host: IpAddr,
    #[default(3306)]
    pub port: u16,
    #[default(String::from("default"))]
    pub dating: String,
    #[serde(rename = "root")]
    pub user: String,
    #[serde(rename = "pass")]
    #[default(String::from(""))]
    pub password: String,
    #[default(Connections::default())]
    pub connections: Connections,
}


#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[allow(unused)]
pub struct Db {
    pub  mysql: MySql,
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[allow(unused)]
pub struct App {
    pub level: LogLevel,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[allow(unused)]
pub struct Log {
    pub app: App,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[derive(SmartDefault)]
#[allow(unused)]
pub struct WatchDog {
    #[serde(with = "humantime_serde")]
    #[default(Duration::from_secs(5))]
    pub period: Duration,
    #[default(10)]
    pub limit: u16,
    #[serde(with = "humantime_serde")]
    #[default(Duration::from_secs(4))]
    pub lock_timeout: Duration,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[allow(unused)]
pub struct Backgroud {
    pub watchdog: WatchDog,
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