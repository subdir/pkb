use std::fmt;


use Value;
use Suit;

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Hash)]
pub struct Card {
    value: Value,
    suit: Suit,
}


impl Card {
    pub const fn deck_size() -> usize {
        Value::VARIANTS_NUM * Suit::VARIANTS_NUM
    }

    pub fn gen_deck() -> impl Iterator<Item=Card> {
        (0 .. Self::deck_size()).map(|x| Card::from_serial(x))
        /*
        Bug: https://github.com/rust-lang/rust/issues/38615

        use sequential::{Sequential, LowBound};

        Value::lowest().sequence().flat_map(|v|
            Suit::VARIANTS.iter().map(|s| Card::new(v, Suit::Hearts))
        )
        */
    }

    pub fn new(value: Value, suit: Suit) -> Card {
        Self { value: value, suit: suit }
    }

    pub fn from_str(s: &str) -> Self {
        assert_eq!(s.len(), 2);
        let mut chars = s.chars();
        Card::new(
            Value::from_char(chars.next().unwrap()),
            Suit::from_char(chars.next().unwrap()),
        )
    }

    fn from_serial(num: usize) -> Self {
        assert!(num < Self::deck_size());
        Card::new(
            Value::from_serial(num / Suit::VARIANTS_NUM),
            Suit::from_serial(num % Suit::VARIANTS_NUM)
        )
    }

    pub fn value(self) -> Value { self.value }
    pub fn suit(self) -> Suit { self.suit }

    pub fn to_string(self) -> String {
        format!("{}{}", self.value().to_char(), self.suit().to_char())
    }
}


impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}


#[cfg(test)]
mod tests {
    use itertools::join;
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Card::from_str("AC"), Card::new(Value::Ace, Suit::Clubs));
        assert_eq!(
            join(Card::gen_deck().map(|c| c.to_string()), ":"),
            concat!(
                "2S:2H:2D:2C:3S:3H:3D:3C:4S:4H:4D:4C:5S:5H:5D:5C:",
                "6S:6H:6D:6C:7S:7H:7D:7C:8S:8H:8D:8C:9S:9H:9D:9C:",
                "TS:TH:TD:TC:JS:JH:JD:JC:QS:QH:QD:QC:KS:KH:KD:KC:",
                "AS:AH:AD:AC"
            )
        );
    }
}

