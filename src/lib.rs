#[macro_use]
mod utils;

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

#[derive(Debug)]
pub struct Cards {
    cards: Vec<Card>,
}

impl Cards {
    fn new_deck() -> Self {
        let mut deck = Cards { cards: Vec::with_capacity(Value::all().len() * Suit::all().len()) };
        for value in Value::all() {
            for suit in Suit::all() {
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
