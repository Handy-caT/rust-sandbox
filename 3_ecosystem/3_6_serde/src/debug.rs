use std::time::{Duration};
use duration_str::deserialize_duration;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Debug {
    #[serde(deserialize_with = "deserialize_duration")]
    duration: Duration,
    at: String,
}

#[cfg(test)]
mod tests {
    use std::time::{Duration};
    use crate::debug::Debug;

    #[test]
    fn debug_parse() {
        let data = r#"{
            "duration": "1m",
            "at": "2019-06-28T08:35:46+00:00"
        }"#;

        let t: Debug = serde_json::from_str(data).unwrap();

        assert_eq!(t.duration, Duration::from_secs(60));
        assert_eq!(t.at, "2019-06-28T08:35:46+00:00");
    }
}