use std::fmt::Debug;
use std::pin::Pin;

pub trait SayHi: Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

pub trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        // Implementation must be meaningful, and
        // obviously call something requiring `&mut self`.
        // The point here is to practice dealing with
        // `Pin<&mut Self>` -> `&mut self` conversion
        // in different contexts, without introducing
        // any `Unpin` trait bounds.
    }
}