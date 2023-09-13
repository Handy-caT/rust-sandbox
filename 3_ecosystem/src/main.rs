mod rgba_wrapper;

use std::fs::File;
use image::{GenericImageView, ImageFormat};
use imagequant::RGBA;
use crate::rgba_wrapper::RGBAWrapper;


async fn load_image_from_path<S: AsRef<str>>(path: S) -> RGBAWrapper {
    let bytes = tokio::fs::read(path.as_ref()).await.unwrap();
    RGBAWrapper::new(&bytes)
}

#[tokio::main]
async fn main() {
    //let img = load_image_from_path("output.jpg").await;
    //let img = load_image_from_path("beautiful-shot-grassy-hills-covered-trees-near-mountains-dolomites-italy.jpg").await;
    let img = load_image_from_path("dental-implants-surgery-concept-pen-tool-created-clipping-path-included-jpeg-easy-composite.jpg").await;
    // The dimensions method returns the images width and height.


    let mut liq = imagequant::new();
    liq.set_speed(3).unwrap();
    liq.set_quality(70, 100).unwrap();

    let width = img.width;
    let height = img.height;


    let mut liq_image = liq.new_image_borrowed(&img, width as usize, height as usize, 0.0).unwrap();

    let mut res = match liq.quantize(&mut liq_image) {
        Ok(res) => res,
        Err(err) => panic!("Quantization failed, because: {err:?}"),
    };

    res.set_dithering_level(1.0).unwrap();

    let (palette, pixels) = res.remapped(&mut liq_image).unwrap();

    let mut img = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let index = pixels[(y * width + x) as usize];
        let color = palette[index as usize];
        *pixel = image::Rgba([color.r, color.g, color.b, color.a]);
    }

    //tokio::fs::write("output1.jpg", img.as_raw()).await.unwrap();

    let mut output = File::create("output1.jpg").unwrap();
    img.write_to(&mut output, ImageFormat::Jpeg).unwrap();
}
