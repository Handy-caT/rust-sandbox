use std::path::PathBuf;
use std::str::Bytes;
use std::thread;
use imagequant::{Attributes, RGBA};
use std::time::Instant;
use crate::rgba_wrapper::RGBAWrapper;

pub fn get_liq() -> Attributes {
    let mut liq = imagequant::new();
    liq.set_speed(3).unwrap();
    liq.set_quality(70, 100).unwrap();

    liq
}

async fn get_bytes_from_file(file: PathBuf) -> Vec<u8> {
    tokio::fs::read(file)
        .await
        .unwrap()
}

async fn get_bytes_from_url(url: url::Url) -> Vec<u8> {
    reqwest::get(url)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap()
        .to_vec()
}


pub async fn process_image(buffer: &[u8], start: Instant) -> ((Vec<RGBA>, Vec<u8>), u32, u32) {
    println!("Started quantization {:?}", start.elapsed());
    let img = RGBAWrapper::new(buffer);

    let liq = get_liq();

    let width = img.width;
    let height = img.height;

    let bitmap: Vec<RGBA> = img.into();
    //let mut liq_image = liq.new_image_borrowed(bitmap.as_slice(), width as usize, height as usize, 0.0).unwrap();

    let res = thread::spawn(move || {
        let mut liq_image = liq.new_image_borrowed(bitmap.as_slice(), width as usize, height as usize, 0.0).unwrap();

        let mut res = match liq.quantize(&mut liq_image) {
            Ok(res) => res,
            Err(err) => panic!("Quantization failed, because: {err:?}"),
        };

        res.set_dithering_level(1.0).unwrap();
        res.remapped(&mut liq_image).unwrap()
    }).join().unwrap();

    println!("Finished quantization {:?}", start.elapsed());

    (res, width, height)
}

