use bytemuck::{cast_vec};
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
    image: image::DynamicImage,
    pub width: u32,
    pub height: u32
}

impl RGBAWrapper {
    pub fn new(bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        let width = image.width();
        let height = image.height();

        Self {
            width,
            height,
            image
        }
    }
}

impl Into<Vec<RGBA>> for RGBAWrapper {
    fn into(self) -> Vec<RGBA> {
       cast_vec(self.image.to_rgba8().to_vec())
    }
}


