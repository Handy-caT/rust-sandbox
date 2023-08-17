use std::fmt::Debug;
use std::pin::Pin;
use crate::pins::base_traits::{MutMeSomehow, SayHi};

impl<T> SayHi for Vec<T>
where T: Debug {

}

impl<T> MutMeSomehow for Vec<T>
where T: Debug + Default + Unpin {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let this = self.get_mut();
        println!("mut_me_somehow: before: {:?}", this);
        this.push(T::default());
        println!("mut_me_somehow: after: {:?}", this);
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;
    use crate::pins::base_struct::Point;
    use crate::pins::base_traits::{MutMeSomehow, SayHi};

    #[test]
    fn test_vec_hi() {
        let b = vec![Point::new(1, 2), Point::new(3, 4)];
        let p = Pin::new(&b);
        p.say_hi();
    }

    #[test]
    fn test_vec_mut_me_somehow() {
        let mut b = vec![Point::new(1, 2), Point::new(3, 4)];
        let p = Pin::new(&mut b);
        p.mut_me_somehow();
        assert_eq!(b.len(), 3);
        assert_eq!(b[2].x, 0);
        assert_eq!(b[2].y, 0);
    }
}