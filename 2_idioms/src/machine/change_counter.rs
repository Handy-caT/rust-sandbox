use std::collections::HashMap;
use crate::coin::coin::Coin;

pub fn count_sum(coins: &[Coin]) -> u16 {
    let mut sum = 0;
    for coin in coins {
        sum += coin.value();
    }
    sum
}

pub fn find_nearest(purse: &HashMap<Coin, u8>, amount: u16) -> Option<Coin> {
    let mut nearest = None;
    let mut min_diff = 50;

    for (coin, coin_amount) in purse {
        let diff = amount.wrapping_sub(coin.value());
        if diff < min_diff && *coin_amount > 0 {
            min_diff = diff;
            nearest = Some(*coin);
        }
    }

    nearest
}

pub fn change_counter(purse: &HashMap<Coin, u8>, change_amount: u16) -> Option<Vec<Coin>> {
    let mut change_amount = change_amount;
    let mut coin_vec = Vec::<Coin>::new();
    let mut fail = false;

    while change_amount > 0 {
        let nearest = find_nearest(purse, change_amount);
        if let Some(coin) = nearest {
            coin_vec.push(coin);
            change_amount -= coin.value();
        } else {
            fail = true;
            change_amount = 0;
        }
    }

    if fail {
        None
    } else {
        Some(coin_vec)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::coin::coin::Coin;
    use crate::machine::change_counter::{change_counter, find_nearest};

    #[test]
    fn test_find_nearest() {
        let purse = HashMap::from([
            (Coin::One(), 1),
            (Coin::Two(), 1),
            (Coin::Five(), 1),
            (Coin::Ten(), 1),
            (Coin::Twenty(), 1),
            (Coin::Fifty(), 1),
        ]);

        assert_eq!(find_nearest(&purse, 1), Some(Coin::One()));
        assert_eq!(find_nearest(&purse, 3), Some(Coin::Two()));
    }

    #[test]
    fn test_find_nearest_none() {
        let purse = HashMap::from([
            (Coin::One(), 0),
            (Coin::Two(), 0),
            (Coin::Five(), 1),
            (Coin::Ten(), 1),
            (Coin::Twenty(), 1),
            (Coin::Fifty(), 1),
        ]);

        assert_eq!(find_nearest(&purse, 1),None);
        assert_eq!(find_nearest(&purse, 3), None);

    }

    #[test]
    fn test_change_counter() {
        let purse = HashMap::from([
            (Coin::One(), 10),
            (Coin::Two(), 10),
            (Coin::Five(), 10),
            (Coin::Ten(), 10),
            (Coin::Twenty(), 10),
            (Coin::Fifty(), 10),
        ]);

        let change = change_counter(&purse, 1);

        assert!(change.is_some());
        assert_eq!(change.unwrap(), vec![Coin::One()]);

        let change = change_counter(&purse, 13);

        assert!(change.is_some());
        assert_eq!(change.unwrap(), vec![Coin::Ten(), Coin::Two(), Coin::One()]);
    }

    #[test]
    fn test_change_counter_little() {
        let purse = HashMap::from([
            (Coin::One(), 10),
            (Coin::Two(), 10),
            (Coin::Five(), 0),
            (Coin::Ten(), 0),
            (Coin::Twenty(), 0),
            (Coin::Fifty(), 0),
        ]);

        let change = change_counter(&purse, 1);

        assert!(change.is_some());
        assert_eq!(change.unwrap(), vec![Coin::One()]);

        let change = change_counter(&purse, 13);

        assert!(change.is_some());
        let change_vec = change.unwrap();
        assert_eq!(change_vec.len(), 7);
    }

    #[test]
    fn test_change_counter_no_change() {
        let purse = HashMap::from([
            (Coin::One(), 0),
            (Coin::Two(), 0),
            (Coin::Five(), 10),
            (Coin::Ten(), 10),
            (Coin::Twenty(), 10),
            (Coin::Fifty(), 10),
        ]);

        let change = change_counter(&purse, 1);
        assert!(change.is_none());

        let change = change_counter(&purse, 13);
        assert!(change.is_none());
    }
}