use tracing::Level;
use tracing_subscriber::fmt::format::{Format, JsonFields};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub struct Logger {}

impl Logger {
    pub fn init() {
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
            .with(layer)
            .init();
    }
}