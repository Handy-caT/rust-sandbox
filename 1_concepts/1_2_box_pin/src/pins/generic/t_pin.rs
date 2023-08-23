use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use crate::pins::base_traits::{MutMeSomehow, SayHi};

impl<T> SayHi for T where T: Debug {
}

impl<T> MutMeSomehow for T where T: Debug + Default + Future {
    fn mut_me_somehow(self: Pin<&mut Self>) {

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