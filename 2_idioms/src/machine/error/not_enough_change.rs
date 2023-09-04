use std::fmt::Debug;

pub struct NotEnoughChange {}

impl std::fmt::Display for NotEnoughChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self::Debug::fmt(self, f)
    }
}

impl Debug for NotEnoughChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not enough change")
    }
}

impl std::error::Error for NotEnoughChange {}

impl NotEnoughChange {
    pub fn new() -> Self {
        Self {}
    }
}