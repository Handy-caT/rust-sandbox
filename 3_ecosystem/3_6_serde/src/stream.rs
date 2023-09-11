use serde::{Deserialize, Serialize};
use uuid::Uuid;
use url::Url;
use crate::private_tariff::PrivateTariff;
use crate::public_tariff::PublicTariff;

#[derive(Serialize, Deserialize, Debug)]
pub struct Stream {
    user_id: Uuid,
    is_private: bool,
    settings: u32,
    shard_url: Url,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[cfg(test)]
mod tests {
    use crate::stream::Stream;

    #[test]
    fn stream_parse() {
        let data = r#"
        {
            "user_id": "8d234120-0bda-49b2-b7e0-fbd3912f6cbf",
            "is_private": false,
            "settings": 45345,
            "shard_url": "https://n3.example.com/sapi",
            "public_tariff": {
                "id": 1,
                "price": 100,
                "duration": "1h",
                "description": "test public tariff"
            },
            "private_tariff": {
                "client_price": 100,
                "duration": "1m",
                "description": "test private tariff"
            }
        }
        "#;

        let s: Stream = serde_json::from_str(data).unwrap();

        assert_eq!(s.user_id.to_string(), "8d234120-0bda-49b2-b7e0-fbd3912f6cbf");
        assert_eq!(s.is_private, false);
        assert_eq!(s.settings, 45345);
        assert_eq!(s.shard_url.to_string(), "https://n3.example.com/sapi");

    }
}