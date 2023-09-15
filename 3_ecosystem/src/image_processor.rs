use std::path::PathBuf;
use std::thread;
use imagequant::{Attributes, RGBA};
use image::ImageFormat;
use std::fs::File;
use tracing::{info, span};
use tracing::Level;
use url::Url;
use crate::rgba_wrapper::RGBAWrapper;

pub fn get_liq(min_quality: u8, target_quality: u8) -> Attributes {
    let mut liq = imagequant::new();
    liq.set_speed(3).unwrap();
    liq.set_quality(min_quality, target_quality).unwrap();

    liq
}

async fn get_bytes_from_file(file: &PathBuf) -> Vec<u8> {
    tokio::fs::read(file)
        .await
        .unwrap()
}

fn get_filename_from_file(file: &PathBuf) -> String {
    file.file_name().unwrap().to_str().unwrap().to_string()
}

fn get_filename_from_url(url: &Url) -> String {
    url.path_segments().unwrap().last().unwrap().to_string()
}

async fn get_bytes_from_url(url: &Url) -> Vec<u8> {
    reqwest::get(url.clone())
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap()
        .to_vec()
}


pub async fn process_image(buffer: &[u8], liq: Attributes) -> ((Vec<RGBA>, Vec<u8>), u32, u32) {
    let span = span!(target: "process", Level::TRACE, "process_image");
    let _enter = span.enter();
    let img = RGBAWrapper::new(buffer);

    let width = img.width;
    let height = img.height;

    let bitmap: Vec<RGBA> = img.into();

    let res = thread::spawn(move || {
        let mut liq_image = liq.new_image_borrowed(bitmap.as_slice(), width as usize, height as usize, 0.0).unwrap();

        let mut res = match liq.quantize(&mut liq_image) {
            Ok(res) => res,
            Err(err) => panic!("Quantization failed, because: {err:?}"),
        };

        res.set_dithering_level(1.0).unwrap();
        res.remapped(&mut liq_image).unwrap()
    }).join().unwrap();

    (res, width, height)
}

pub fn save_image(res: (Vec<RGBA>, Vec<u8>), width: u32, height: u32, filename: String) {
    let pixels = res.1;
    let palette = res.0;

    let mut img = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let index = pixels[(y * width + x) as usize];
        let color = palette[index as usize];
        *pixel = image::Rgba([color.r, color.g, color.b, color.a]);
    }

    thread::spawn(move ||
        {
            let mut output = File::create(format!("output/{}", filename)).unwrap();
            img.write_to(&mut output, ImageFormat::Jpeg).unwrap();
        }
    ).join().unwrap();

}

pub struct ImageProcessor {}

impl ImageProcessor {

    pub async fn process_url_image(url: Url, liq: Attributes) {
        let bytes = get_bytes_from_url(&url).await;
        let start = std::time::Instant::now();
        let (res, width, height) = process_image(&bytes, liq).await;
        info!("Processing took {:?} for {}", start.elapsed(), get_filename_from_url(&url));

        save_image(res, width, height, get_filename_from_url(&url));
    }

    pub async fn process_file_image( file: PathBuf, liq: Attributes) {
        let bytes = tokio::fs::read(&file).await.unwrap();
        let start = std::time::Instant::now();
        let (res, width, height) = process_image(&bytes, liq).await;
        info!("Processing took {:?} for {}", start.elapsed(), get_filename_from_file(&file));

        save_image(res, width, height, get_filename_from_file(&file));
    }
}