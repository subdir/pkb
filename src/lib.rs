#![feature(const_fn)]

#[macro_use]
mod xenum;

mod suit;
mod value;
mod rank;

pub use suit::Suit;
pub use value::Value;
pub use rank::Rank;


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Hash)]
pub struct Card {
    value: Value,
    suit: Suit,
}


impl Card {
    pub fn from_str(s: &str) -> Self {
        assert_eq!(s.len(), 2);
        let mut chars = s.chars();
        Card {
            value: Value::from_char(chars.next().unwrap()),
            suit: Suit::from_char(chars.next().unwrap()),
        }
    }

    pub fn to_string(self) -> String {
        format!("{}{}", self.value.to_char(), self.suit.to_char())
    }
}


impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}{}", self.value.to_char(), self.suit.to_char())
    }
}


#[derive(Debug)]
pub struct Cards {
    cards: Vec<Card>,
}


impl Cards {
    pub fn new_deck() -> Self {
        let mut deck = Cards { cards: Vec::with_capacity(Value::variants_num() * Suit::variants_num()) };
        for value in Value::variants() {
            for suit in Suit::variants() {
                deck.cards.push(Card { value: *value, suit: *suit });
            }
        }
        deck
    }

    pub fn from_str(s: &str) -> Self {
        let mut seen = std::collections::HashSet::new();
        let mut cards = Vec::new();

        for s in s.split(":") {
            let card = Card::from_str(s);
            if seen.contains(&card) {
                panic!("Duplicate card {}", card)
            } else {
                seen.insert(card);
                cards.push(card);
            }
        }
        Cards { cards: cards }
    }

    pub fn to_string(&self) -> String {
        self.cards.iter().map(|card| card.to_string()).collect::<Vec<String>>().join(":")
    }

/*
    fn rank(&self) {
        let value_stats = ValueStats::from_cards(*self);
        let suit_stats = SuitStats::from_cards(*self);

        if value.stats
    }*/
}


impl std::fmt::Display for Cards {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Cards::new_deck().to_string(),
            concat!(
                "2S:2H:2D:2C:3S:3H:3D:3C:4S:4H:4D:4C:5S:5H:5D:5C:",
                "6S:6H:6D:6C:7S:7H:7D:7C:8S:8H:8D:8C:9S:9H:9D:9C:",
                "TS:TH:TD:TC:JS:JH:JD:JC:QS:QH:QD:QC:KS:KH:KD:KC:",
                "AS:AH:AD:AC"
            )
        );
        assert_eq!(
            "AS:5H:2D:3D:4D",
            format!("{}", Cards::from_str("AS:5H:2D:3D:4D"))
        )
    }

    #[test]
    #[should_panic(expected = "Duplicate card AS")]
    fn test_duplicate() {
        let cards = Cards::from_str("AS:2H:AS");
    }

    #[test]
    #[should_panic(expected = "No variant for 'X'")]
    fn test_wrong_card() {
        let cards = Cards::from_str("AS:2H:XS");
    }
}
