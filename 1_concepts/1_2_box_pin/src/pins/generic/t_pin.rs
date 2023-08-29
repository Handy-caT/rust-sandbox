use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use crate::pins::base_traits::{MutMeSomehow, SayHi};
use futures::task::noop_waker;

impl<T> SayHi for T where T: Debug {
}

impl<T> MutMeSomehow for T where T: Debug + Default + Future {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let waker = noop_waker();
        let mut cx = std::task::Context::from_waker(&waker);

        self.poll(&mut cx);
    }
}



#[cfg(test)]
mod tests {
    use std::pin::Pin;
    use crate::pins::base_struct::Point;
    use crate::pins::base_traits::{MutMeSomehow, SayHi};

    #[test]
    fn test_point_hi() {
        let b = Point::new(1, 2);
        let p = Pin::new(&b);
        p.say_hi();
    }

}