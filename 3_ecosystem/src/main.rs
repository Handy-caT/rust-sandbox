mod rgba_wrapper;
mod image_processor;
mod file_parser;

use std::fs::File;
use image::{ImageFormat};
use crate::image_processor::{process_image};
use std::time::Instant;


async fn load_image_from_path<S: AsRef<str>>(path: S, start: Instant) {
    let bytes = tokio::fs::read(path.as_ref()).await.unwrap();
    let (res, width, height) = process_image(&bytes, start).await;

    let pixels = res.1;
    let palette = res.0;

    let mut img = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let index = pixels[(y * width + x) as usize];
        let color = palette[index as usize];
        *pixel = image::Rgba([color.r, color.g, color.b, color.a]);
    }

    let mut output = File::create("output1.jpg").unwrap();
    img.write_to(&mut output, ImageFormat::Jpeg).unwrap();
}

fn main() {
    let start = Instant::now();
    //let img = load_image_from_path("output.jpg").await;
    //let img = load_image_from_path("beautiful-shot-grassy-hills-covered-trees-near-mountains-dolomites-italy.jpg").await;
    //load_image_from_path("dental-implants-surgery-concept-pen-tool-created-clipping-path-included-jpeg-easy-composite.jpg", start).await;
    //tokio::fs::write("output1.jpg", img.as_raw()).await.unwrap();

    let default_parallelism_approx = 1;

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(default_parallelism_approx)
        .enable_all()
        .build()
        .unwrap();

    let tasks = vec![
        load_image_from_path("dental-implants-surgery-concept-pen-tool-created-clipping-path-included-jpeg-easy-composite.jpg", start),
        load_image_from_path("dental-implants-surgery-concept-pen-tool-created-clipping-path-included-jpeg-easy-composite.jpg", start)
    ];

    let handles = tasks
        .into_iter()
        .map(|task| runtime.spawn(task))
        .collect::<Vec<_>>();


    runtime.block_on(async {
        for handle in handles {
            handle.await.unwrap();
        }
    });

    println!("Time elapsed: {:?}", start.elapsed());
}
