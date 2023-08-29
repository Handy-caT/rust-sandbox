use std::fmt::Debug;
use std::ops::Deref;
use std::pin::Pin;
use std::rc::Rc;
use crate::pins::base_traits::{MutMeSomehow, SayHi};

impl<T> SayHi for Rc<T>
where T: Debug {
}

impl<T> MutMeSomehow for Rc<T>
where T: Default + Debug {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let this = self.get_mut();
        println!("mut_me_somehow: before: {:?}", this);
        *this = Rc::new(T::default());
        println!("mut_me_somehow: after: {:?}", this);
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;
    use std::rc::Rc;
    use crate::pins::base_struct::Point;
    use crate::pins::base_traits::{MutMeSomehow, SayHi};

    #[test]
    fn test_rc_hi() {
        let b = Rc::new(Point::new(1, 2));
        let p = Pin::new(&b);
        p.say_hi();
    }

    #[test]
    fn test_rc_mut_me_somehow() {
        let mut b = Rc::new(Point::new(1, 2));
        let p = Pin::new(&mut b);
        p.mut_me_somehow();
        println!("after mut_me_somehow: {:?}", b);
        assert_eq!(b.x, 0);
        assert_eq!(b.y, 0);

    }
}