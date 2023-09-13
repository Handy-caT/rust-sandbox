mod rgba_wrapper;
mod image_processor;

use std::fs::File;
use std::io::Cursor;
use image::{GenericImageView, ImageFormat};
use imagequant::RGBA;
use crate::image_processor::get_liq;
use crate::rgba_wrapper::RGBAWrapper;
use image::io::Reader as ImageReader;
use rgb::ComponentBytes;


async fn load_image_from_path<S: AsRef<str>>(path: S) -> RGBAWrapper {
    let bytes = tokio::fs::read(path.as_ref()).await.unwrap();
    RGBAWrapper::new(&bytes)
}

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    //let img = load_image_from_path("output.jpg").await;
    //let img = load_image_from_path("beautiful-shot-grassy-hills-covered-trees-near-mountains-dolomites-italy.jpg").await;
    let img = load_image_from_path("dental-implants-surgery-concept-pen-tool-created-clipping-path-included-jpeg-easy-composite.jpg").await;
    // The dimensions method returns the images width and height.

    let liq = get_liq();

    let width = img.width;
    let height = img.height;

    let bitmap: Vec<RGBA> = img.into();
    let mut liq_image = liq.new_image_borrowed(bitmap.as_slice(), width as usize, height as usize, 0.0).unwrap();

    println!("Finished loading image {:?}", start.elapsed());

    let mut res = match liq.quantize(&mut liq_image) {
        Ok(res) => res,
        Err(err) => panic!("Quantization failed, because: {err:?}"),
    };

    res.set_dithering_level(1.0).unwrap();

    println!("Finished quantization {:?}", start.elapsed());

    let (palette, pixels) = res.remapped(&mut liq_image).unwrap();

    let mut img = ImageReader::new(Cursor::new(palette.as_bytes()))
        .set_format(ImageFormat::Jpeg);



    //tokio::fs::write("output1.jpg", img.as_raw()).await.unwrap();

    let mut output = File::create("output1.jpg").unwrap();
    img.write_to(&mut output, ImageFormat::Jpeg).unwrap();

    println!("Time elapsed: {:?}", start.elapsed());
}
