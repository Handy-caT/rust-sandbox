use std::time::Duration;
use duration_str::deserialize_duration;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Id(pub u64);
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Price(pub u64);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PublicTariff {
    id: Id,
    price: Price,
    #[serde(deserialize_with = "deserialize_duration")]
    duration: Duration,
    description: String,
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::public_tariff::{Id, Price, PublicTariff};

    #[test]
    fn public_tariff_parse() {
        let data = r#"{
            "id": 1,
            "price": 100,
            "duration": "1h",
            "description": "test public tariff"
        }"#;

        let t: PublicTariff = serde_json::from_str(data).unwrap();

        assert_eq!(t.id, Id(1));
        assert_eq!(t.price, Price(100));
        assert_eq!(t.duration, Duration::from_secs(3600));
        assert_eq!(t.description, "test public tariff");
    }
}
