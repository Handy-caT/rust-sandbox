
use std::fmt::{Debug, Formatter};
use crate::machine::error::not_enough_money::NotEnoughMoney;
use crate::machine::error::bad_price::BadPrice;
use crate::machine::error::not_enough_capacity::NotEnoughCapacity;
use crate::machine::error::not_enough_change::NotEnoughChange;
use crate::machine::error::product_not_found::ProductNotFound;

pub enum VendingError<'a> {
    ProductNotFound(ProductNotFound<'a>),
    NotEnoughMoney(NotEnoughMoney<'a>),
    BadPrice(BadPrice<'a>),
    NotEnoughChange(NotEnoughChange),
    NotEnoughCapacity(NotEnoughCapacity),
}

impl<'a> Debug for VendingError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VendingError::ProductNotFound(err) => {
                ProductNotFound::fmt(err, f)
            }
            VendingError::NotEnoughMoney(err) => {
                NotEnoughMoney::fmt(err, f)
            }
            VendingError::NotEnoughChange(err) => {
                NotEnoughChange::fmt(err, f)
            }
            VendingError::NotEnoughCapacity(err) => {
                NotEnoughCapacity::fmt(err, f)
            }
            VendingError::BadPrice(err) =>{
                BadPrice::fmt(err, f)
            }
        }
    }
}

impl<'a> std::fmt::Display for VendingError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self::Debug::fmt(self, f)
    }
}