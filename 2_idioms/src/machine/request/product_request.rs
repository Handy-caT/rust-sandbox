use crate::coin::coin::Coin;
use crate::machine::product::Product;
use crate::machine::request::request_state::{Accepted, CheckedAvailability, CheckedPayment, Created, Denied};

pub struct ProductRequest<'a, State> {
    product: Product<'a>,
    coins: Vec<Coin>,
    state: State,
}

impl<'a, State> ProductRequest<'a, State> {
    pub fn product(&self) -> &Product<'a> {
        &self.product
    }

    pub fn coins(&self) -> &[Coin] {
        &self.coins
    }
}

impl<'a> ProductRequest<'a, Created> {
    pub fn new(product: Product<'a>, coins: Vec<Coin>) -> Self {
        Self {
            product,
            coins,
            state: Created(),
        }
    }

    pub fn check_availability(self) -> ProductRequest<'a, CheckedAvailability> {
        self.into()
    }
}

impl<'a> ProductRequest<'a, CheckedAvailability> {
    pub fn deny(self) -> ProductRequest<'a, Denied> {
        self.into()
    }

    pub fn check_payment(self) -> ProductRequest<'a, CheckedPayment> {
         self.into()
    }
}

impl<'a> ProductRequest<'a, CheckedPayment> {
    pub fn deny(self) -> ProductRequest<'a, Denied> {
        self.into()
    }

    pub fn accept(self) -> ProductRequest<'a, Accepted> {
        self.into()
    }
}

impl<'a> From<ProductRequest<'a, CheckedAvailability>> for ProductRequest<'a, Denied> {
    fn from(request: ProductRequest<'a, CheckedAvailability>) -> Self {
        Self {
            product: request.product,
            coins: request.coins,
            state: Denied(),
        }
    }
}



impl<'a> From<ProductRequest<'a, Created>> for ProductRequest<'a, CheckedAvailability> {
    fn from(request: ProductRequest<'a, Created>) -> Self {
        Self {
            product: request.product,
            coins: request.coins,
            state: CheckedAvailability(),
        }
    }
}

impl<'a> From<ProductRequest<'a, CheckedAvailability>> for ProductRequest<'a, CheckedPayment> {
    fn from(request: ProductRequest<'a, CheckedAvailability>) -> Self {
        Self {
            product: request.product,
            coins: request.coins,
            state: CheckedPayment(),
        }
    }
}

impl<'a> From<ProductRequest<'a, CheckedPayment>> for ProductRequest<'a, Denied> {
    fn from(request: ProductRequest<'a, CheckedPayment>) -> Self {
        Self {
            product: request.product,
            coins: request.coins,
            state: Denied(),
        }
    }
}

impl<'a> From<ProductRequest<'a, CheckedPayment>> for ProductRequest<'a, Accepted> {
    fn from(request: ProductRequest<'a, CheckedPayment>) -> Self {
        Self {
            product: request.product,
            coins: request.coins,
            state: Accepted(),
        }
    }
}