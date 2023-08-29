use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use crate::machine::helpers::Price;

#[derive(Debug, Clone, Eq)]
pub struct Product<'a> {
    name: Cow<'a, str>,
    price: Price,
}

impl<'a> PartialEq for Product<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<'a> Hash for Product<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<'a> Product<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(name: S, price: Price) -> Self {
        Self {
            name: name.into(),
            price,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn price(&self) -> Price {
        self.price
    }
}