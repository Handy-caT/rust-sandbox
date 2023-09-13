use std::future::Future;
use imagequant::Attributes;
use crate::rgba_wrapper::RGBAWrapper;

pub fn get_liq() -> Attributes {
    let mut liq = imagequant::new();
    liq.set_speed(3).unwrap();
    liq.set_quality(70, 100).unwrap();

    liq
}


async fn process_image(buffer: &[u8]) {
    let wrapper = RGBAWrapper::new(buffer);

}


struct ImageProcessor {

}

impl Future for ImageProcessor {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        todo!()
    }
}