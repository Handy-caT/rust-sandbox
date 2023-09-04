use std::fmt::Debug;
use crate::machine::product::Product;

pub struct ProductNotFound<'a> {
    pub product: Product<'a>
}

impl<'a> std::fmt::Display for ProductNotFound<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self::Debug::fmt(self, f)
    }
}

impl<'a> Debug for ProductNotFound<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} not found", self.product.name())
    }
}

impl<'a> std::error::Error for ProductNotFound<'a> {}

impl<'a> ProductNotFound<'a> {
    pub fn new(product: Product<'a>) -> Self {
        Self {
            product
        }
    }
}