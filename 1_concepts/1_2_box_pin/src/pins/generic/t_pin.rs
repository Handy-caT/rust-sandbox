use std::fmt::Debug;
use std::pin::Pin;
use crate::pins::base_traits::{MutMeSomehow, SayHi};

impl<T> SayHi for T where T: Debug {
}

impl<T> MutMeSomehow for T where T: Debug + Default {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let this = unsafe { self.get_unchecked_mut() };
        println!("mut_me_somehow: before: {:?}", this);
        *this = T::default();
        println!("mut_me_somehow: after: {:?}", this);
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

    #[test]
    fn test_point_mut_me_somehow() {
        let mut b = Point::new(1, 2);
        let p = Pin::new(&mut b);
        p.mut_me_somehow();
        assert_eq!(b.x, 0);
        assert_eq!(b.y, 0);
    }
}