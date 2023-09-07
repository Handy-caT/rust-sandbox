use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;

pub trait Merge {
    fn merge(&mut self, another: Self);
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Mode {
    pub debug: Option<bool>
}

impl Merge for Mode {
    fn merge(&mut self, another: Self) {
        if another.debug.is_some() {
            self.debug = another.debug;
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self {
            debug: Some(false),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Server {
    pub external_url: Option<String>,
    pub http_port: Option<u16>,
    pub grpc_port: Option<u16>,
    pub healthz_port: Option<u16>,
    pub metrics_port: Option<u16>,
}

impl Merge for Server {
    fn merge(&mut self, another: Self) {
        if another.external_url.is_some() {
            self.external_url = another.external_url;
        }
        if another.http_port.is_some() {
            self.http_port = another.http_port;
        }
        if another.grpc_port.is_some() {
            self.grpc_port = another.grpc_port;
        }
        if another.healthz_port.is_some() {
            self.healthz_port = another.healthz_port;
        }
        if another.metrics_port.is_some() {
            self.metrics_port = another.metrics_port;
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Self {
            external_url: Some(String::from("http://127.0.0.1")),
            http_port: Some(8081),
            grpc_port: Some(8082),
            healthz_port: Some(10025),
            metrics_port: Some(9199),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Connections {
    pub max_idle: Option<u16>,
    pub max_open: Option<u16>,
}

impl Merge for Connections {
    fn merge(&mut self, another: Self) {
        if another.max_idle.is_some() {
            self.max_idle = another.max_idle;
        }
        if another.max_open.is_some() {
            self.max_open = another.max_open;
        }
    }
}

impl Default for Connections {
    fn default() -> Self {
        Self {
            max_idle: Some(30),
            max_open: Some(30),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct MySql {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub dating: Option<String>,
    pub user: Option<String>,
    #[serde(rename = "pass")]
    pub password: Option<String>,
    pub connections: Option<Connections>,
}

impl Merge for MySql {
    fn merge(&mut self, another: Self) {
        if another.host.is_some() {
            self.host = another.host;
        }
        if another.port.is_some() {
            self.port = another.port;
        }
        if another.dating.is_some() {
            self.dating = another.dating;
        }
        if another.user.is_some() {
            self.user = another.user;
        }
        if another.password.is_some() {
            self.password = another.password;
        }
        if another.connections.is_some() {
            self.connections = another.connections;
        }
    }
}

impl Default for MySql {
    fn default() -> Self {
        Self {
            host: Some(String::from("127.0.0.1")),
            port: Some(3306),
            dating: Some(String::from("default")),
            user: Some(String::from("root")),
            password: Some(String::from("")),
            connections: Some(Connections::default()),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Db {
    pub  mysql: Option<MySql>,
}

impl Merge for Db {
    fn merge(&mut self, another: Self) {
        if another.mysql.is_some() {
            self.mysql = another.mysql;
        }
    }
}

impl Default for Db {
    fn default() -> Self {
        Self {
            mysql: Some(MySql::default()),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub enum LogLevel {
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    Warn,
    Error,
    Trace,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct App {
    pub level: Option<LogLevel>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            level: Some(LogLevel::default()),
        }
    }
}

impl Merge for App {
    fn merge(&mut self, another: Self) {
        if another.level.is_some() {
            self.level = another.level;
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Log {
    pub app: Option<App>,
}

impl Default for Log {
    fn default() -> Self {
        Self {
            app: Some(App::default()),
        }
    }
}

impl Merge for Log {
    fn merge(&mut self, another: Self) {
        if another.app.is_some() {
            self.app = another.app;
        }
    }
}



#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WatchDog {
    pub period: Option<String>,
    pub limit: Option<u16>,
    pub lock_timeout: Option<String>,
}

impl Merge for WatchDog {
    fn merge(&mut self, another: Self) {
        if another.period.is_some() {
            self.period = another.period;
        }
        if another.limit.is_some() {
            self.limit = another.limit;
        }
        if another.lock_timeout.is_some() {
            self.lock_timeout = another.lock_timeout;
        }
    }
}

impl Default for WatchDog {
    fn default() -> Self {
        Self {
            period: Some(String::from("5s")),
            limit: Some(10),
            lock_timeout: Some(String::from("4s")),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Backgroud {
    pub watchdog: Option<WatchDog>,
}

impl Merge for Backgroud {
    fn merge(&mut self, another: Self) {
        if another.watchdog.is_some() {
            self.watchdog = another.watchdog;
        }
    }
}

impl Default for Backgroud {
    fn default() -> Self {
        Self {
            watchdog: Some(WatchDog::default()),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct Settings {
    pub mode: Mode,
    pub server: Server,
    pub db: Db,
    pub log: Log,
    pub background: Backgroud,
}


impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            // Add in `./Settings.toml`
            .add_source(File::with_name("config"))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .build()
            .unwrap();

        let from_env = Self::from_env();
        let settings_right = settings.try_deserialize();
        match settings_right {
            Ok(settings) => {
                let mut default_settings = Settings::default();
                default_settings.merge(settings);

                if let Ok(settings) = from_env {
                    default_settings.merge(settings);
                }

                Ok(default_settings)
            },
            Err(e) => Err(e),
        }
    }

    fn from_env() -> Result<Self, ConfigError> {
        let env_mode = Config::builder()
            .add_source(Environment::with_prefix("CONF_MODE"))
            .build()
            .unwrap();

        let mode = env_mode.try_deserialize::<Mode>()?;

        let env_server = Config::builder()
            .add_source(Environment::with_prefix("CONF_SERVER"))
            .build()
            .unwrap();

        let server = env_server.try_deserialize::<Server>()?;

        let env_db = Config::builder()
            .add_source(Environment::with_prefix("CONF_DB_MYSQL"))
            .build()
            .unwrap();

        let mut db = env_db.try_deserialize::<Db>()?;

        let env_connection = Config::builder()
            .add_source(Environment::with_prefix("CONF_DB_MYSQL_CONNECTIONS"))
            .build()
            .unwrap();

        let connections = env_connection.try_deserialize::<Connections>()?;
        if db.mysql.is_some() {
            db.mysql.as_mut().unwrap().connections = Some(connections);
        }

        let env_log = Config::builder()
            .add_source(Environment::with_prefix("CONF_LOG_APP"))
            .build()
            .unwrap();

        let log = env_log.try_deserialize::<Log>()?;

        let env_watchdog = Config::builder()
            .add_source(Environment::with_prefix("CONF_BACKGROUND_WATCHDOG"))
            .build()
            .unwrap();

        let watchdog = env_watchdog.try_deserialize::<WatchDog>()?;

        let background = Backgroud {
            watchdog: Some(watchdog),
        };

        Ok(Settings {
            mode,
            server,
            db,
            log,
            background,
        })

    }
}

impl Merge for Settings {
    fn merge(&mut self, another: Self) {
        self.mode.merge(another.mode);
        self.server.merge(another.server);
        self.db.merge(another.db);
        self.log.merge(another.log);
        self.background.merge(another.background);
    }
}

