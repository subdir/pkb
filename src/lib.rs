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
pub struct Card {
    value: Value,
    suit: Suit,
}


impl Card {
    fn from_str(s: &str) -> Self {
        assert_eq!(s.len(), 2);
        let mut chars = s.chars();
        Card {
            value: Value::from_char(chars.next().unwrap()),
            suit: Suit::from_char(chars.next().unwrap()),
        }
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
    fn new_deck() -> Self {
        let mut deck = Cards { cards: Vec::with_capacity(Value::variants_num() * Suit::variants_num()) };
        for value in Value::variants() {
            for suit in Suit::variants() {
                deck.cards.push(Card { value: *value, suit: *suit });
            }
        }
        deck
    }

    fn from_strs(strs: &[&str]) -> Self {
        let mut cards = Vec::with_capacity(strs.len());
        for s in strs {
            cards.push(Card::from_str(s));
        }
        Cards { cards: cards }
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
        for card in self.cards.iter() {
            try! { card.fmt(f) };
            try! { write!(f, ":") };
        }
        Ok(())
    }
}


#[test]
fn test() {
    assert_eq!(
        "AS:TH:2D:3D:4D:",
        format!("{}", Cards::from_strs(&["AS", "TH", "2D", "3D", "4D"]))
    )
}
