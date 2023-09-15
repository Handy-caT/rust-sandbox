use tracing::Level;
use tracing_subscriber::filter::{LevelFilter, Targets};
use tracing_subscriber::fmt::format::{Format, JsonFields};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::util::SubscriberInitExt;
use crate::settings::Settings;

pub struct Logger {}

impl Logger {
    pub fn init(settings: &Settings) {
        let global_level: LevelFilter = settings.log_config.level.into();
        let process_level: LevelFilter = settings.log_config.level_process.into();

        let stderr = std::io::stderr.with_max_level(Level::WARN);
        let layer =  tracing_subscriber::fmt::Layer::new()
            .event_format(Format::default()
                .json()
                .flatten_event(true)
            )
            .fmt_fields(JsonFields::new())
            .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
            .map_writer(move |out| stderr.or_else(out))
            .with_target(false)
            .with_filter(global_level);

        let process_layer = tracing_subscriber::fmt::Layer::new()
            .event_format(Format::default()
                .json()
                .flatten_event(true)
            )
            .fmt_fields(JsonFields::new())
            .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
            .map_writer(move |out| stderr.or_else(out))
            .with_target(false)
            .with_filter(Targets::new().with_target("process", Level::DEBUG))
            .with_filter(process_level);


        tracing_subscriber::registry()
            .with(process_layer)
            .with(layer)
            .init();
    }
}