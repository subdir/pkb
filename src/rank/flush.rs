use std::fmt;

use sequential::{Sequential, LowBound};
use rank::highcard::HighCard;


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Flush {
    values: HighCard,
}


impl Flush {
    pub fn new(values: HighCard) -> Self {
        Self { values: values }
    }
    pub fn ranks_count() -> usize { HighCard::ranks_count() }
}


impl LowBound for Flush {
    fn lowest() -> Self { Self { values: HighCard::lowest() } }
}


impl Sequential for Flush {
    fn consequent(&self) -> Option<Self> {
        self.values.consequent().map(|v| Self::new(v))
    }
}


impl fmt::Display for Flush {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}*", self.values)
    }
}

