use std::fmt;

use value::Value;
use sequential::{Sequential, LowBound};
use rank::Straight;
use rank::distinct_ordered_five::DistinctOrderedFive;
use rank::combinations_count;

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct HighCard {
    values: DistinctOrderedFive,
}


impl HighCard {
    pub fn new(high: Value, kicker1: Value, kicker2: Value, kicker3: Value, kicker4: Value) -> Self {
        Self { values: DistinctOrderedFive::from_values(
            high,
            kicker1,
            kicker2,
            kicker3,
            kicker4
        )}
    }

    pub fn ranks_count() -> usize {
        combinations_count(5, Value::VARIANTS_NUM) - Straight::ranks_count() + 1
    }
}

impl LowBound for HighCard {
    fn lowest() -> Self {
        Self{ values: DistinctOrderedFive::lowest().skip_straight().unwrap() }
    }
}


impl Sequential for HighCard {
    fn consequent(&self) -> Option<Self> {
        self.values
        .consequent()
        .and_then(|c| c.skip_straight())
        .map(|c| Self { values: c })
    }
}


impl fmt::Display for HighCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.values)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ranks_count() {
        assert_eq!(HighCard::ranks_count(), HighCard::lowest().sequence().count());
    }
}
