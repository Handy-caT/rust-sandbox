use std::pin::Pin;
use crate::pins::base_traits::{MutMeSomehow, SayHi};

impl SayHi for &[u8] {

}

impl MutMeSomehow for &[u8] {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let this = self.get_mut();
        println!("mut_me_somehow: before: {:?}", this);

        println!("mut_me_somehow: after: {:?}", this);
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;
    use crate::pins::base_traits::SayHi;

    #[test]
    fn test_slice_hi() {
        let v = vec![1, 2, 3];
        let b = &v[..];
        let p = Pin::new(&b);
        p.say_hi();
    }
}