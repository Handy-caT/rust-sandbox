use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Id(pub u64);
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Price(pub u64);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Gift {
    id: Id,
    price: Price,
    description: String,
}

#[cfg(test)]
mod tests {
    use crate::gift::{Id, Price, Gift};

    #[test]
    fn gift_parse() {
        let data = r#"{
            "id": 1,
            "price": 100,
            "description": "test gift"
        }"#;

        let t: Gift = serde_json::from_str(data).unwrap();

        assert_eq!(t.id, Id(1));
        assert_eq!(t.price, Price(100));
        assert_eq!(t.description, "test gift");
    }
}