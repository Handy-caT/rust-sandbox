use crate::request::Request;

mod request;
mod stream;
mod public_tariff;
mod private_tariff;
mod gift;
mod debug;

fn main() {
    let data = r#"
        {
  "type": "success",
  "stream": {
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
      "client_price": 250,
      "duration": "1m",
      "description": "test private tariff"
    }
  },
  "gifts": [{
    "id": 1,
    "price": 2,
    "description": "Gift 1"
  }, {
    "id": 2,
    "price": 3,
    "description": "Gift 2"
  }],
  "debug": {
    "duration": "234ms",
    "at": "2019-06-28T08:35:46+00:00"
  }
}

                    "#;

    let r: Request = serde_json::from_str(data).unwrap();

    println!("{:?}", r);

    let toml = toml::to_string(&r).unwrap();

    println!("{}", toml);

    let yaml = serde_yaml::to_string(&r).unwrap();

    println!("{}", yaml);
}
