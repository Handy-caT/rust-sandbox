use std::collections::HashSet;
use std::path::PathBuf;
use smart_default::SmartDefault;
use std::thread::available_parallelism;
use clap::ArgMatches;
use config::{Config, ConfigError, File, Map, Source, Value};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(SmartDefault, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LevelFilter {
    Off,
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl Into<tracing_subscriber::filter::LevelFilter> for LevelFilter {
    fn into(self) -> tracing::level_filters::LevelFilter {
        match self {
            LevelFilter::Off => tracing::level_filters::LevelFilter::OFF,
            LevelFilter::Trace => tracing::level_filters::LevelFilter::TRACE,
            LevelFilter::Debug => tracing::level_filters::LevelFilter::DEBUG,
            LevelFilter::Info => tracing::level_filters::LevelFilter::INFO,
            LevelFilter::Warn => tracing::level_filters::LevelFilter::WARN,
            LevelFilter::Error => tracing::level_filters::LevelFilter::ERROR,
        }
    }
}


#[derive(SmartDefault, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    #[default(Some(PathBuf::from("urls.txt")))]
    pub file: Option<PathBuf>,
    #[default(available_parallelism().unwrap().get())]
    pub max_threads: usize,
    #[default(HashSet::new())]
    pub urls: HashSet<Url>,
    #[default(HashSet::new())]
    pub files: HashSet<PathBuf>,
    #[default(false)]
    pub interactive: bool,
    #[default(70)]
    pub min_quality: u8,
    #[default(99)]
    pub target_quality: u8,
    #[default(PathBuf::from("config.toml"))]
    pub config: PathBuf,
    #[default(LoggingSettings::default())]
    pub log_config: LoggingSettings,
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoggingSettings {
    #[default(LevelFilter::Info)]
    pub level: LevelFilter,
    #[default(LevelFilter::Info)]
    pub level_process: LevelFilter,
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
    pub fn new(filename: PathBuf) -> Self {
        let config = Config::builder()
            .add_source(Settings::default())
            .add_source(File::from(filename).required(false))
            .build()
            .expect("Failed to build configuration");

        config.try_deserialize().expect("Failed to deserialize Settings")
    }

    pub fn parse_args(&mut self, matches: &ArgMatches) {

        let file = matches.get_one::<PathBuf>("file");
        let max_threads = matches.get_one::<usize>("max_threads");


        if file.is_some() {
            self.file = Some(file.unwrap().clone());
        }

        if max_threads.is_some() {
            self.max_threads = max_threads.unwrap().clone();
        }

        let urls = matches.get_many::<Url>("urls");
        if urls.is_some() {
            self.urls = urls.unwrap().into_iter().cloned().collect::<HashSet<_>>()
        }

        let files = matches.get_many::<PathBuf>("files");
        if files.is_some() {
            self.files = files.unwrap().into_iter().cloned().collect::<HashSet<_>>()
        }

        self.interactive = matches.get_flag("interactive");
    }
}