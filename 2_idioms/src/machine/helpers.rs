use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub struct Price(pub u8);

impl Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub struct Quantity(pub u8);

impl Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}