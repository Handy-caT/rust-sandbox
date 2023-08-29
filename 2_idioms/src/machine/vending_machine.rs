use std::collections::HashMap;
use std::mem;
use crate::coin::coin::Coin;
use crate::machine::change_counter::{change_counter, count_sum};
use crate::machine::error::bad_price::BadPrice;
use crate::machine::error::not_enough_capacity::NotEnoughCapacity;
use crate::machine::error::not_enough_change::NotEnoughChange;
use crate::machine::error::not_enough_money::NotEnoughMoney;
use crate::machine::error::product_not_found::ProductNotFound;
use crate::machine::error::vending_error::VendingError;
use crate::machine::helpers::{Price, Quantity};
use crate::machine::product::Product;
use crate::machine::request::product_request::ProductRequest;


struct VendingMachine<'a> {
    products: HashMap<Product<'a>, (Price, Quantity)>,
    purse: HashMap<Coin, u8>,
    max_capacity: u8,
    available_capacity: u8,
}

impl<'a> VendingMachine<'a> {
    pub fn new(max_capacity: u8) -> Self {
        let mut machine = Self {
            products: HashMap::new(),
            purse: HashMap::new(),
            max_capacity,
            available_capacity: max_capacity,
        };

        machine.add_coin_count(Coin::One(), 10);
        machine.add_coin_count(Coin::Two(), 10);
        machine.add_coin_count(Coin::Five(), 10);
        machine.add_coin_count(Coin::Ten(), 10);
        machine.add_coin_count(Coin::Twenty(), 10);

        machine
    }

    pub fn add_product(&mut self, product: Product<'a>, quantity: Quantity) -> Result<(), VendingError> {
        if let std::collections::hash_map::Entry::Vacant(e) = self.products.entry(product.clone()) {
            let price = product.price();
            e.insert((price, quantity));
            self.available_capacity -= quantity.0;
        } else {
            let (price, current_quantity) = self.products.get_mut(&product).unwrap();
            if price != &product.price() {
                return Err(VendingError::BadPrice(BadPrice::new(product.price().0 as u16, product.clone())));
            }
            if self.available_capacity < quantity.0 {
                return Err(VendingError::NotEnoughCapacity(NotEnoughCapacity::new(self.available_capacity as u16, quantity.0 as u16)));
            }
            current_quantity.0 += quantity.0;
            self.available_capacity -= quantity.0;
        }
        Ok(())
    }


    pub fn max_capacity(&self) -> u8 {
        self.max_capacity
    }

    fn add_coins(&mut self, coins: &[Coin]) {
        for coin in coins {
            let coin_amount = self.purse.entry(*coin).or_insert(0);
            *coin_amount += 1;
        }
    }

    fn delete_coins(&mut self, coins: &[Coin]) {
        for coin in coins {
            let coin_amount = self.purse.entry(*coin).or_insert(0);
            *coin_amount -= 1;
        }
    }

    fn add_coin_count(&mut self, coin: Coin, count: u8) {
        let coin_amount = self.purse.entry(coin).or_insert(0);
        *coin_amount += count;
    }

    fn check_availability(&self, product: &Product) -> bool {
        self.products.get(product).map_or(false, |quantity| quantity.1.0 > 0)
    }

    fn get_price_products(products: &HashMap<Product, (Price, Quantity)>, product: &Product<'a>) -> Result<Price, VendingError<'a>> {
        products.get(product).map_or(Err(VendingError::ProductNotFound(ProductNotFound::new(product.clone()))), |quantity| Ok(quantity.0))
    }

    pub fn get_price(&self, product: &Product<'a>) -> Result<Price, VendingError> {
        VendingMachine::get_price_products(&self.products, product)
    }

    pub fn get_products(&self) -> Vec<Product> {
        self.products.keys().cloned().collect()
    }

    fn give_product(&mut self, product: &Product<'a>) -> Result<(), VendingError> {
        let (_, quantity) = self.products.get_mut(product).unwrap();
        quantity.0 -= 1;
        self.available_capacity += 1;
        Ok(())
    }

    pub fn buy_product(&mut self, product: &Product<'a>, coins: Vec<Coin>) -> Result<(Product<'a>, Vec<Coin>), (VendingError, Vec<Coin>)> {
        self.add_coins(&coins);
        let request = ProductRequest::new(product.clone(), coins);

        let request = request.check_availability();
        if !self.check_availability(request.product()) {
            return Err((VendingError::ProductNotFound(ProductNotFound::new(request.product().clone())), Vec::from(request.coins())));
        }

        let change_amount = {
            let coins_sum = count_sum(request.coins());
            let products = mem::take(&mut self.products);
            let price = VendingMachine::get_price_products(&products, request.product());
            if price.is_err() {
                return Err((price.unwrap_err(), Vec::from(request.coins())));
            }
            let price = price.unwrap().0;

            let _ = mem::replace(&mut self.products, products);
            if coins_sum < price as u16 {
                return Err((VendingError::NotEnoughMoney(NotEnoughMoney::new(coins_sum, request.product().clone())), Vec::from(request.coins())));
            }

            coins_sum - price as u16
        };
        let request = request.check_payment();

        let change = change_counter(&self.purse, change_amount);
        if change.is_none() {
            self.delete_coins(request.coins());
            return Err((VendingError::NotEnoughChange(NotEnoughChange::new()), Vec::from(request.coins())));
        }

        let change = change.unwrap();
        self.delete_coins(&change);

        let request = request.accept();

        let res = self.give_product(request.product());
        if res.is_err() {
            return Err((res.unwrap_err(), Vec::from(request.coins())));
        }

        Ok((request.product().clone(), change))
    }

}

#[cfg(test)]
mod tests {
    use crate::coin::coin::Coin;
    use crate::machine::change_counter::count_sum;
    use crate::machine::helpers::{Price, Quantity};
    use crate::machine::product::Product;
    use crate::machine::vending_machine::VendingMachine;

    #[test]
    fn test_new() {
        let vending_machine = VendingMachine::new(10);
        assert_eq!(vending_machine.max_capacity(), 10);
        assert_eq!(vending_machine.available_capacity, 10);
        assert_eq!(vending_machine.purse.get(&Coin::One()), Some(&10));
        assert_eq!(vending_machine.purse.get(&Coin::Two()), Some(&10));
        assert_eq!(vending_machine.purse.get(&Coin::Five()), Some(&10));
        assert_eq!(vending_machine.purse.get(&Coin::Ten()), Some(&10));
        assert_eq!(vending_machine.purse.get(&Coin::Twenty()), Some(&10));
    }

    #[test]
    fn test_add_product() {
        let mut vending_machine = VendingMachine::new(10);
        let product = Product::new("Coca-Cola", Price(10));
        vending_machine.add_product(product, Quantity(5));
        assert_eq!(vending_machine.max_capacity(), 10);
        assert_eq!(vending_machine.available_capacity, 5);
    }

    #[test]
    fn test_add_product_bad_price() {
        let mut vending_machine = VendingMachine::new(10);
        let product = Product::new("Coca-Cola", Price(10));
        vending_machine.add_product(product, Quantity(5));

        let product = Product::new("Coca-Cola", Price(5));

        let result = vending_machine.add_product(product, Quantity(5));
        assert!(result.is_err());
        assert_eq!(vending_machine.max_capacity(), 10);
        assert_eq!(vending_machine.available_capacity, 5);
    }

    #[test]
    fn test_add_product_not_enough_capacity() {
        let mut vending_machine = VendingMachine::new(10);
        let product = Product::new("Coca-Cola", Price(10));
        vending_machine.add_product(product, Quantity(5));

        let product = Product::new("Coca-Cola", Price(10));

        let result = vending_machine.add_product(product, Quantity(6));
        assert!(result.is_err());
        assert_eq!(vending_machine.max_capacity(), 10);
        assert_eq!(vending_machine.available_capacity, 5);
    }

    #[test]
    fn test_add_coins() {
        let mut vending_machine = VendingMachine::new(10);
        let coins = vec![Coin::One(), Coin::Two(), Coin::Five()];
        vending_machine.add_coins(&coins);
        assert_eq!(vending_machine.max_capacity(), 10);
        assert_eq!(vending_machine.purse.get(&Coin::One()), Some(&11));
        assert_eq!(vending_machine.purse.get(&Coin::Two()), Some(&11));
        assert_eq!(vending_machine.purse.get(&Coin::Five()), Some(&11));
    }

    #[test]
    fn test_add_coin_count() {
        let mut vending_machine = VendingMachine::new(10);
        vending_machine.add_coin_count(Coin::One(), 5);
        assert_eq!(vending_machine.max_capacity(), 10);
        assert_eq!(vending_machine.purse.get(&Coin::One()), Some(&15));
    }

    #[test]
    fn test_check_availability() {
        let mut vending_machine = VendingMachine::new(10);
        let product = Product::new("Coca-Cola", Price(10));
        vending_machine.add_product(product.clone(), Quantity(5));

        assert!(vending_machine.check_availability(&product));

        let product_other_price = Product::new("Coca-Cola", Price(20));

        assert!(vending_machine.check_availability(&product_other_price));
    }

    #[test]
    fn test_get_price() {
        let mut vending_machine = VendingMachine::new(10);
        let product = Product::new("Coca-Cola", Price(10));
        vending_machine.add_product(product.clone(), Quantity(5));

        let price = vending_machine.get_price(&product);
        assert!(price.is_ok());
        assert_eq!(price.unwrap(), Price(10));
    }

    #[test]
    fn test_get_products() {
        let mut vending_machine = VendingMachine::new(10);
        let product = Product::new("Coca-Cola", Price(10));
        vending_machine.add_product(product.clone(), Quantity(5));

        let products = vending_machine.get_products();
        assert_eq!(products.len(), 1);
        assert_eq!(products[0], product);
    }

    #[test]
    fn test_delete_coins() {
        let mut vending_machine = VendingMachine::new(10);

        let coins = vec![Coin::One(), Coin::Two(), Coin::Five()];
        vending_machine.delete_coins(&coins);

        assert_eq!(vending_machine.max_capacity(), 10);
        assert_eq!(vending_machine.purse.get(&Coin::One()), Some(&9));
        assert_eq!(vending_machine.purse.get(&Coin::Two()), Some(&9));
        assert_eq!(vending_machine.purse.get(&Coin::Five()), Some(&9));
    }

    #[test]
    fn test_buy_product_ok() {
        let mut vending_machine = VendingMachine::new(10);

        let product = Product::new("Coca-Cola", Price(10));
        vending_machine.add_product(product.clone(), Quantity(5));

        let coins = vec![Coin::Two(), Coin::Two(), Coin::Two(), Coin::Five()];

        let result = vending_machine.buy_product(&product, coins);

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result.0, product);
        assert_eq!(result.1, vec![Coin::One()]);

        assert_eq!(vending_machine.max_capacity(), 10);
        assert_eq!(vending_machine.available_capacity, 6);
    }

    #[test]
    fn test_buy_product_ok_more_coins() {
        let mut vending_machine = VendingMachine::new(10);

        let product = Product::new("Coca-Cola", Price(10));
        vending_machine.add_product(product.clone(), Quantity(5));

        let coins = vec![Coin::Two(), Coin::Two(), Coin::Two(), Coin::Five(), Coin::Ten()];

        let result = vending_machine.buy_product(&product, coins);

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result.0, product);
        assert_eq!(count_sum(&result.1), 11);

        assert_eq!(vending_machine.max_capacity(), 10);
        assert_eq!(vending_machine.available_capacity, 6);
    }

    #[test]
    fn test_buy_product_no_product() {
        let mut vending_machine = VendingMachine::new(10);

        let product = Product::new("Coca-Cola", Price(10));
        let other_product = Product::new("Pepsi", Price(10));
        vending_machine.add_product(product.clone(), Quantity(5));

        let coins = vec![Coin::Two(), Coin::Two(), Coin::Two(), Coin::Five(), Coin::Ten()];

        let result = vending_machine.buy_product(&other_product, coins);

        assert!(result.is_err());
        let result = result.unwrap_err();

        assert_eq!(result.1, vec![Coin::Two(), Coin::Two(), Coin::Two(), Coin::Five(), Coin::Ten()]);

        assert_eq!(vending_machine.max_capacity(), 10);
        assert_eq!(vending_machine.available_capacity, 5);
    }
}