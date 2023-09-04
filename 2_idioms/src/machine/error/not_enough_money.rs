use std::fmt;
use std::fmt::{Debug, Formatter};
use crate::machine::product::Product;

pub struct NotEnoughMoney<'a> {
    pub price: u16,
    pub product: Product<'a>
}

impl<'a> fmt::Display for NotEnoughMoney<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self::Debug::fmt(self, f)
    }
}

impl<'a> Debug for NotEnoughMoney<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Not enough money for: {} for {}. {} price is {}",
               self.price, self.product.name(), self.product.name(), self.product.price())
    }
}

impl<'a> std::error::Error for NotEnoughMoney<'a> {}


impl<'a> NotEnoughMoney<'a> {
    pub fn new(price: u16, product: Product<'a>) -> Self {
        Self {
            price,
            product
        }
    }
}

