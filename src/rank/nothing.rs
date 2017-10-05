use std::fmt;

use sequential::Sequential;
use rank::distinct_five::DistinctFive;


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Nothing {
    cards: DistinctFive,
}


impl Nothing {
    pub fn lowest() -> Self {
        Self{ cards: DistinctFive::lowest().skip_straight().unwrap() }
    }
}


impl Sequential for Nothing {
    fn consequent(&self) -> Option<Self> {
        match self.cards.consequent().and_then(|c| c.skip_straight()) {
            None => None,
            Some(cards) => Some(Self { cards: cards })
        }
    }
}


impl fmt::Display for Nothing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cards)
    }
}

