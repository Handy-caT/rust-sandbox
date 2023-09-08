use std::fs;
use std::ops::Deref;
use tracing::{info, Level, warn};
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::format::{Format, JsonFields};
use tracing_subscriber::Layer;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;


fn main() {

    let file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("access.log")
        .unwrap();

    let local =  tracing_subscriber::fmt::Layer::new()
        .event_format(Format::default()
            .json()
            .flatten_event(true)
        )
        .fmt_fields(JsonFields::new())
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
        .with_target(false)
        .with_writer(file)
        .with_filter(Targets::new().with_target("local", Level::DEBUG));

    let stderr = std::io::stderr.with_max_level(Level::WARN);
    let layer =  tracing_subscriber::fmt::Layer::new()
        .event_format(Format::default()
            .json()
            .flatten_event(true)
        )
        .fmt_fields(JsonFields::new())
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
        .map_writer(move |out| stderr.or_else(out))
        .with_target(false);

    tracing_subscriber::registry()
        .with(local)
        .with(layer)
        .init();

    info!(msg = "Starting application");
    warn!(msg = "Something is not right");

    warn!(target: "local", msg = "Something is not right");

}
