use std::fmt;
use std::fmt::{Debug, Formatter};
use crate::machine::product::Product;

pub struct NotEnoughMoneyError<'a> {
    pub price: u16,
    pub product: Product<'a>
}

impl<'a> fmt::Display for NotEnoughMoneyError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self::Debug::fmt(self, f)
    }
}

impl<'a> Debug for NotEnoughMoneyError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Bad price: {} for {}. {} price is {}",
               self.price, self.product.name(), self.product.name(), self.product.price())
    }
}

impl<'a> std::error::Error for NotEnoughMoneyError<'a> {}


impl<'a> NotEnoughMoneyError<'a> {
    pub fn new(price: u16, product: Product<'a>) -> Self {
        Self {
            price,
            product
        }
    }
}

