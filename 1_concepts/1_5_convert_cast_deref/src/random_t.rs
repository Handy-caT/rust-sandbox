use std::ops::Deref;
use rand::Rng;

struct Random<T>(T, T, T);

impl<T> Random<T> {
    pub fn new(a: T, b: T, c: T) -> Self {
        Random(a, b, c)
    }
}

impl<T> Deref for Random<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..3);
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => &self.2,
        }
    }
}