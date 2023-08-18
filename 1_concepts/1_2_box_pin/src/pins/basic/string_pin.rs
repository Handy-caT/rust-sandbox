use std::pin::Pin;
use crate::pins::base_traits::{MutMeSomehow, SayHi};

impl SayHi for String {
}

impl MutMeSomehow for String {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let this = self.get_mut();
        println!("mut_me_somehow: before: {:?}", this);
        this.push_str(" world");
        println!("mut_me_somehow: after: {:?}", this);
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;
    use crate::pins::base_traits::{MutMeSomehow, SayHi};

    #[test]
    fn test_string_hi() {
        let b = String::from("hello");
        let p = Pin::new(&b);
        p.say_hi();
    }

    #[test]
    fn test_string_mut_me_somehow() {
        let mut b = String::from("hello");
        let p = Pin::new(&mut b);
        p.mut_me_somehow();
        assert_eq!(b, "hello world");
    }
}