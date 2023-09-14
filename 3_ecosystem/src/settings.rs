use std::collections::HashSet;
use std::path::PathBuf;
use smart_default::SmartDefault;
use std::thread::available_parallelism;
use clap::ArgMatches;
use url::Url;

#[derive(SmartDefault, Debug, Clone, PartialEq)]
pub struct Settings {
    #[default(Some(PathBuf::from("urls.txt")))]
    pub file: Option<PathBuf>,
    #[default(available_parallelism().unwrap().get())]
    pub max_threads: usize,
    #[default(HashSet::new())]
    pub urls: HashSet<Url>,
    #[default(HashSet::new())]
    pub files: HashSet<PathBuf>
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse_args(matches: &ArgMatches) -> Self {

        let file = matches.get_one::<PathBuf>("file");
        let max_threads = matches.get_one::<usize>("max_threads");

        let mut settings = Self::new();

        if file.is_some() {
            settings.file = Some(file.unwrap().clone());
        }

        if max_threads.is_some() {
            settings.max_threads = max_threads.unwrap().clone();
        }

        let urls = matches.get_many::<Url>("urls");
        if urls.is_some() {
            settings.urls = urls.unwrap().into_iter().cloned().collect::<HashSet<_>>()
        }

        let files = matches.get_many::<PathBuf>("files");
        if files.is_some() {
            settings.files = files.unwrap().into_iter().cloned().collect::<HashSet<_>>()
        }

        settings
    }
}