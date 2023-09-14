mod rgba_wrapper;
mod image_processor;
mod file_parser;
mod logger;
mod cli;
mod settings;

use std::collections::HashSet;
use std::path::PathBuf;
use std::thread::available_parallelism;
use crate::image_processor::{ImageProcessor};
use tracing::info;
use url::Url;
use crate::cli::CLI;
use crate::file_parser::FileParser;
use crate::logger::Logger;
use crate::settings::Settings;

async fn get_urls_and_files(settings: Settings) -> (HashSet<Url>, HashSet<PathBuf>) {
    if settings.file.is_some() {
        let fileparser = FileParser::new(settings.file.unwrap());
        fileparser.parse_file().await
    } else {
        (settings.urls, settings.files)
    }
}

fn main() {

    let matches = CLI::parse_args();
    let settings = Settings::parse_args(&matches);

    Logger::init();
    info!("Starting the program");
    let start = std::time::Instant::now();
    info!("Start time: {:?}", start);

    let default_parallelism_approx = available_parallelism().unwrap().get();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(default_parallelism_approx)
        .enable_all()
        .build()
        .unwrap();

    let (urls, files) = runtime.block_on(get_urls_and_files(settings));

    info!("Urls: {:?}", urls);
    info!("Files: {:?}", files);

    let tasks_url = urls
        .into_iter()
        .map(ImageProcessor::process_url_image)
        .collect::<Vec<_>>();

    let tasks_file = files
        .into_iter()
        .map(ImageProcessor::process_file_image)
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

    runtime.block_on(async {
        for handle in handles {
            handle.await.unwrap();
        }
    });

    info!("Total time: {:?}", start.elapsed());
}
