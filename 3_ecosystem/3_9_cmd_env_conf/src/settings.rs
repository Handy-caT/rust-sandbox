use serde_derive::Deserialize;


#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Mode {
    default: bool
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Server {
    external_url: String,
    http_port: u16,
    grpc_port: u16,
    healthz_port: u16,
    metrics_port: u16,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Connections {
    max_idle: u16,
    max_open: u16,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct MySql {
    host: String,
    port: u16,
    dating: String,
    user: String,
    password: String,
    connections: Connections,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Db {
    mysql: MySql,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Trace,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct App {
    level: LogLevel,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Log {
    app: App,
}

struct WatchDog {
    period:
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Settings {
    mode: Mode,
    server: Server,
    db: Db,
    log: Log,
}