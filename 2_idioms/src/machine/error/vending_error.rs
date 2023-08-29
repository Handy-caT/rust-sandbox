use std::error::Error;
use std::fmt::{Debug, Formatter};
use crate::machine::error::bad_price_error::NotEnoughMoneyError;

pub enum VendingError<'a> {
    ProductNotFound,
    NotEnoughMoney(NotEnoughMoneyError<'a>),
    BadPrice,
    NotEnoughChange,
    NotEnoughCapacity,
}

impl<'a> Debug for VendingError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VendingError::ProductNotFound => write!(f, "Product not found"),
            VendingError::NotEnoughMoney(err) => {
                NotEnoughMoneyError::fmt(err, f)
            }
            VendingError::NotEnoughChange => write!(f, "Not enough change"),
            VendingError::NotEnoughCapacity => write!(f, "Not enough capacity"),
            VendingError::BadPrice => write!(f, "Bad price"),
        }
    }
}

impl<'a> std::fmt::Display for VendingError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self::Debug::fmt(self, f)
    }
}