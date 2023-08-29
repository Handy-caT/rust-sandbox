use std::fmt::Debug;
use crate::machine::product::Product;

pub struct BadPrice<'a> {
    pub price: u16,
    pub product: Product<'a>
}

impl<'a> std::fmt::Display for BadPrice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl<'a> Debug for BadPrice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bad price: {} for {}. {} price is {}",
               self.price, self.product.name(), self.product.name(), self.product.price())
    }
}

impl<'a> std::error::Error for BadPrice<'a> {}

impl<'a> BadPrice<'a> {
    pub fn new(price: u16, product: Product<'a>) -> Self {
        Self {
            price,
            product
        }
    }
}