mod rgba_wrapper;
mod image_processor;
mod file_parser;

use std::path::PathBuf;
use crate::image_processor::{ImageProcessor, process_image, save_image};
use std::time::Instant;
use crate::file_parser::FileParser;


async fn load_image_from_path<S: AsRef<str>>(path: S) {
    let bytes = tokio::fs::read(path.as_ref()).await.unwrap();
    let (res, width, height) = process_image(&bytes).await;

    save_image(res, width, height);
}

fn main() {
    let start = Instant::now();
    //let img = load_image_from_path("output.jpg").await;
    //let img = load_image_from_path("beautiful-shot-grassy-hills-covered-trees-near-mountains-dolomites-italy.jpg").await;
    //load_image_from_path("dental-implants-surgery-concept-pen-tool-created-clipping-path-included-jpeg-easy-composite.jpg", start).await;
    //tokio::fs::write("output1.jpg", img.as_raw()).await.unwrap();

    let fileparser = FileParser::new(PathBuf::from("urls.txt"));

    let default_parallelism_approx = 2;

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(default_parallelism_approx)
        .enable_all()
        .build()
        .unwrap();

    let fileparser = FileParser::new(PathBuf::from("urls.txt"));
    let (urls, files) = runtime.block_on(fileparser.parse_file());

    let tasks_url = urls
        .into_iter()
        .map(|url| ImageProcessor::process_url_image(url))
        .collect::<Vec<_>>();

    let tasks_file = files
        .into_iter()
        .map(|file| ImageProcessor::process_file_image(file))
        .collect::<Vec<_>>();

    // let tasks = vec![
    //     ImageProcessor::process_file_image(PathBuf::from("dental-implants-surgery-concept-pen-tool-created-clipping-path-included-jpeg-easy-composite.jpg")),
    //     ImageProcessor::process_file_image(PathBuf::from("dental-implants-surgery-concept-pen-tool-created-clipping-path-included-jpeg-easy-composite.jpg"))
    // ];
    // let handles = tasks
    //     .into_iter()
    //     .map(|task| runtime.spawn(task))
    //     .collect::<Vec<_>>();

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

    println!("Time elapsed: {:?}", start.elapsed());
}
