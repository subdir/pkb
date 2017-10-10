use std::fmt;

use sequential::{Sequential, LowBound};
use value::Value;
use rank::distinct::Distinct;
use rank::permutations_count;


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct FullHouse {
    house: Distinct<Value, Value>
}


impl FullHouse {
    pub fn new(three_card_value: Value, two_card_value: Value) -> Self {
        Self { house: Distinct::new(three_card_value, two_card_value) }
    }

    pub fn three_card_value(&self) -> Value { self.house.primary() }
    pub fn two_card_value(&self) -> Value { self.house.secondary() }

    pub fn ranks_count() -> usize {
        permutations_count(2, Value::VARIANTS_NUM)
    }
}


impl LowBound for FullHouse {
    fn lowest() -> Self { Self::new(Value::Two, Value::Three) }
}


impl Sequential for FullHouse {
    fn consequent(&self) -> Option<Self> {
        self.house.consequent().map(|h| Self { house: h })
    }
}


impl fmt::Display for FullHouse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "{}{}{}{}{}",
            self.three_card_value(),
            self.three_card_value(),
            self.three_card_value(),
            self.two_card_value(),
            self.two_card_value(),
        )
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ranks_count() {
        assert_eq!(FullHouse::ranks_count(), FullHouse::lowest().sequence().count());
    }
}
