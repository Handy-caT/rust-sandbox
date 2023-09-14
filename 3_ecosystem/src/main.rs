mod rgba_wrapper;
mod image_processor;
mod file_parser;
mod logger;
mod cli;
mod settings;
mod stdin_reader;

use std::collections::HashSet;
use std::path::PathBuf;
use clap::ArgMatches;
use imagequant::Attributes;
use crate::image_processor::{get_liq, ImageProcessor};
use tracing::info;
use url::Url;
use crate::cli::CLI;
use crate::file_parser::FileParser;
use crate::logger::Logger;
use crate::settings::Settings;
use crate::stdin_reader::{read_until_empty, StdinReader};

async fn get_urls_and_files(settings: Settings) -> (HashSet<Url>, HashSet<PathBuf>) {
    if settings.interactive {
        let mut reader = StdinReader::new();
        read_until_empty(&mut reader)
    }else if settings.file.is_some() {
        let fileparser = FileParser::new(settings.file.unwrap());
        fileparser.parse_file().await
    } else {
        (settings.urls, settings.files)
    }
}

fn get_handles(urls: HashSet<Url>, files: HashSet<PathBuf>, runtime: &tokio::runtime::Runtime, liq: Attributes) -> Vec<tokio::task::JoinHandle<()>> {

    let tasks_url = urls
        .into_iter()
        .map(|url| ImageProcessor::process_url_image(url, liq.clone()))
        .collect::<Vec<_>>();

    let tasks_file = files
        .into_iter()
        .map(|file| ImageProcessor::process_file_image(file, liq.clone()))
        .collect::<Vec<_>>();

    let mut handles = tasks_url
        .into_iter()
        .map(|task| runtime.spawn(task))
        .collect::<Vec<_>>();

    let mut handles_file = tasks_file
        .into_iter()
        .map(|task| runtime.spawn(task))
        .collect::<Vec<_>>();

    handles.append(&mut handles_file);
    handles
}

fn get_runtime(threads: usize) -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(threads)
        .enable_all()
        .build()
        .unwrap()
}

fn get_filename_from_cli(matches: &ArgMatches) -> PathBuf {
    let filename: Option<&PathBuf> = matches.get_one("config");
    if filename.is_some() {
        filename.unwrap().clone()
    } else {
        Settings::default().config
    }
}

fn main() {
    let matches = CLI::parse_args();
    let mut settings = Settings::new(get_filename_from_cli(&matches));
    settings.parse_args(&matches);

    Logger::init();
    info!("Starting the program");
    info!("Settings: {:?}", settings);
    let start = std::time::Instant::now();
    info!("Start time: {:?}", start);

    let threads_count = settings.max_threads;

    let liq = get_liq(settings.min_quality, settings.target_quality);
    let runtime = get_runtime(threads_count);
    let (urls, files) = runtime.block_on(get_urls_and_files(settings));

    info!("Urls: {:?}", urls);
    info!("Files: {:?}", files);

    let handles = get_handles(urls, files, &runtime, liq);

    runtime.block_on(async {
        for handle in handles {
            handle.await.unwrap();
        }
    });

    info!("Total time: {:?}", start.elapsed());
}
