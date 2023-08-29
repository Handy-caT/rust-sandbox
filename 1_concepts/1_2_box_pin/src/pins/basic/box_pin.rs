use std::fmt::{Debug};
use std::pin::Pin;
use crate::pins::base_traits::{MutMeSomehow, SayHi};


impl<T> SayHi for Box<T> where T: Debug {
}

impl<T> MutMeSomehow for Box<T> where T: Debug + Default {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let this = self.get_mut();
        println!("mut_me_somehow: before: {:?}", this);
        *(*this) = T::default();
        println!("mut_me_somehow: after: {:?}", this);
    }
}


#[cfg(test)]
mod tests {
    use std::pin::Pin;
    use crate::pins::base_struct::Point;
    use crate::pins::base_traits::{MutMeSomehow, SayHi};

    #[test]
    fn test_box_hi() {
        let b = Box::new(Point::new(1, 2));
        let p = Pin::new(&b);
        p.say_hi();
    }

    #[test]
    fn test_box_mut_me_somehow() {
        let mut b = Box::new(Point::new(1, 2));
        let p = Pin::new(&mut b);
        p.mut_me_somehow();
        assert_eq!(b.x, 0);
        assert_eq!(b.y, 0);

    }
}