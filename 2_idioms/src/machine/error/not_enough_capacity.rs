use std::fmt::Debug;

pub struct NotEnoughCapacity {
    pub capacity: u16,
    pub needed: u16,
}

impl std::fmt::Display for NotEnoughCapacity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self::Debug::fmt(self, f)
    }
}

impl Debug for NotEnoughCapacity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not enough capacity: has {} but needed {}", self.capacity, self.needed)
    }
}

impl std::error::Error for NotEnoughCapacity {}

impl NotEnoughCapacity {
    pub fn new(capacity: u16, needed: u16) -> Self {
        Self {
            capacity,
            needed
        }
    }
}