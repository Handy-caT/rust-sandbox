#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
pub enum Coin {
    One(),
    Two(),
    Five(),
    Ten(),
    Twenty(),
    Fifty(),
}

impl Coin {
    pub fn new(value: u16) -> Option<Self> {
        match value {
            1 => Some(Coin::One()),
            2 => Some(Coin::Two()),
            5 => Some(Coin::Five()),
            10 => Some(Coin::Ten()),
            20 => Some(Coin::Twenty()),
            50 => Some(Coin::Fifty()),
            _ => None,
        }
    }

    pub fn value(&self) -> u16 {
        match self {
            Coin::One() => 1,
            Coin::Two() => 2,
            Coin::Five() => 5,
            Coin::Ten() => 10,
            Coin::Twenty() => 20,
            Coin::Fifty() => 50,
        }
    }
}

impl Into<u16> for Coin {
    fn into(self) -> u16 {
        self.value()
    }
}