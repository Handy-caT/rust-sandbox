use std::io::Error;
use std::ops::Deref;
use slog::{Drain, info, Logger, o, OwnedKVList, Record, warn};
use slog::{FnValue, PushFnValue};
use std::sync::Mutex;
use slog_term::{CompactFormat, TermDecorator};


struct RuntimeLevelFilter<D> {
    drain_stdout: D,
    drain_stderr: D,
}

impl<D> RuntimeLevelFilter<D> where D: Drain {
    pub fn new(out: D, err: D) -> Self {
        Self {
            drain_stdout: out,
            drain_stderr: err,
        }
    }
}

impl<D> Drain for RuntimeLevelFilter<D>
where D: Drain {
    type Ok = Option<D::Ok>;
    type Err = Option<D::Err>;

    fn log(&self, record: &Record, values: &OwnedKVList) -> Result<Self::Ok, Self::Err> {
        let level = record.level();

        if level.is_at_least(slog::Level::Warning) {
            self.drain_stderr.log(record, values)
                .map(Some)
                .map_err(Some)?;
        }

        self.drain_stdout.log(record, values)
            .map(Some)
            .map_err(Some)
    }
}

struct AppLogger {
    logger: slog::Logger,
}

impl AppLogger {
    pub fn new() -> Self {
        let drain = slog_json::Json::new(std::io::stdout())
            .build()
            .fuse();
        let drain_stdout = slog_async::Async::new(drain).build().fuse();

        let drain = slog_json::Json::new(std::io::stderr())
            .build()
            .fuse();
        let drain_stderr = slog_async::Async::new(drain).build().fuse();

        let drain = RuntimeLevelFilter::new(drain_stdout, drain_stderr).fuse();


        let logger = slog::Logger::root(
            drain,
            o!(
                "file" => "app.log",
                "time" => FnValue(move |_ : &Record| {
                    time::OffsetDateTime::now_utc()
                    .format(&time::format_description::well_known::Rfc3339)
                    .ok()
            }),
                "level" => FnValue(move |rinfo : &Record| {
                rinfo.level().as_short_str()
            }),
                "msg" => PushFnValue(move |record : &Record, ser| {
                ser.emit(record.msg())
            }),
            )
        );

        Self {
            logger
        }
    }
}

impl AsRef<Logger> for AppLogger {
    fn as_ref(&self) -> &Logger {
        &self.logger
    }
}

impl Deref for AppLogger {
    type Target = Logger;
    fn deref(&self) -> &Self::Target {
        &self.logger
    }
}

struct FileLogger {
    logger: slog::Logger,
}

impl FileLogger {
    pub fn new() -> Self {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("access.log")
            .unwrap();

        let drain = slog_json::Json::new(file)
            .build()
            .fuse();

        let drain = slog_async::Async::new(drain).build().fuse();

        let logger = slog::Logger::root(
            drain,
            o!(
                "file" => "access.log",
                "time" => FnValue(move |_ : &Record| {
                    time::OffsetDateTime::now_utc()
                    .format(&time::format_description::well_known::Rfc3339)
                    .ok()
            }),
                "level" => FnValue(move |rinfo : &Record| {
                rinfo.level().as_short_str()
            }),
                "msg" => PushFnValue(move |record : &Record, ser| {
                ser.emit(record.msg())
            }),
            )
        );

        Self {
            logger
        }

    }
}

impl Deref for FileLogger {
    type Target = Logger;
    fn deref(&self) -> &Self::Target {
        &self.logger
    }
}


fn main() {
    let app_logger = AppLogger::new();

    info!(app_logger.as_ref(), "Starting application");
    warn!(app_logger, "Something is not right");

    let file_logger = FileLogger::new();

    for i in 0..10 {
        info!(file_logger, "Accessing resource {}", i);
    }

}
