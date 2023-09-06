use config::{Config, ConfigError, Environment, File, Map, Source, Value};
use serde_derive::Deserialize;


#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Mode {
    debug: Option<bool>
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Server {
    external_url: Option<String>,
    http_port: Option<u16>,
    grpc_port: Option<u16>,
    healthz_port: Option<u16>,
    metrics_port: Option<u16>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Connections {
    max_idle: Option<u16>,
    max_open: Option<u16>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct MySql {
    host: Option<String>,
    port: Option<u16>,
    dating: Option<String>,
    user: Option<String>,
    #[serde(rename = "pass")]
    password: Option<String>,
    connections: Option<Connections>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Db {
    mysql: Option<MySql>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
enum LogLevel {
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    Warn,
    Error,
    Trace,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct App {
    level: Option<LogLevel>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Log {
    app: Option<App>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct WatchDog {
    period: Option<String>,
    limit: Option<u16>,
    lock_timeout: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Backgroud {
    watchdog: Option<WatchDog>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    mode: Mode,
    server: Server,
    db: Db,
    log: Log,
    background: Backgroud,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            // Add in `./Settings.toml`
            .add_source(File::with_name("config"))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("CONF"))
            .build()
            .unwrap();

        settings.try_deserialize()
    }
}
