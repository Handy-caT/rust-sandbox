use std::time::Duration;
use duration_str::deserialize_duration;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Price(pub u32);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PrivateTariff {
    client_price: Price,
    #[serde(deserialize_with = "deserialize_duration")]
    duration: Duration,
    description: String,
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::private_tariff::{Price, PrivateTariff};

    #[test]
    fn private_tariff_parse() {
        let data = r#"{
            "client_price": 100,
            "duration": "1m",
            "description": "test private tariff"
        }"#;

        let t: PrivateTariff = serde_json::from_str(data).unwrap();

        assert_eq!(t.client_price, Price(100));
        assert_eq!(t.duration, Duration::from_secs(60));
        assert_eq!(t.description, "test private tariff");
    }
}