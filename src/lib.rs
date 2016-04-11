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
/*
    fn rank(&self) {
        let value_stats = ValueStats::from_cards(*self);
        let suit_stats = SuitStats::from_cards(*self);

        if value.stats
    }*/
}
