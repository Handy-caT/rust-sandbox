use std::ops::Deref;
use image::{GenericImageView, Rgba};
use imagequant::RGBA;

fn cast_to_liq_color(pixel: &Rgba<u8>) -> RGBA {
    RGBA {
        r: pixel[0],
        g: pixel[1],
        b: pixel[2],
        a: pixel[3]
    }
}

pub struct RGBAWrapper {
    data: Box<[RGBA]>,
    pub width: u32,
    pub height: u32
}

impl RGBAWrapper {
    pub fn new(bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        Self {
            width: image.width(),
            height: image.height(),
            data: image.to_rgba8().pixels().map(cast_to_liq_color).collect::<Vec<_>>().into_boxed_slice()
        }
    }
}

impl Into<Box<[RGBA]>> for RGBAWrapper {
    fn into(self) -> Box<[RGBA]> {
        self.data
    }
}

impl Deref for RGBAWrapper {
    type Target = [RGBA];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
