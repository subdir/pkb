use std::fmt;

use sequential::Sequential;
use value::Value;
use value::Value::*;
use rank::distinct_ordered::DistinctOrdered;
use rank::distinct_ordered_two::DistinctOrderedTwo;
use rank::distinct_ordered_three::DistinctOrderedThree;


pub type DistinctOrderedFive = DistinctOrdered<DistinctOrderedTwo, DistinctOrderedThree>;


impl DistinctOrderedFive {
    pub fn from_values(highest: Value, high: Value, middle: Value, low: Value, lowest: Value) -> Self {
        Self::new(
            DistinctOrderedTwo::new(highest, high),
            DistinctOrderedThree::new_three(middle, low, lowest)
        )
    }

    pub fn lowest() -> Self {
        Self::from_values(Six, Five, Four, Three, Two)
    }

    pub fn is_straight(&self) -> bool {
        self.higher().is_straight()
        && self.lower().is_straight()
        && self.higher().lower() == self.lower().higher().consequent().unwrap()
    }

    pub fn skip_straight(&self) -> Option<Self> {
        self
        .sequence()
        .skip_while(|f| f.is_straight())
        .next()
    }
}


impl fmt::Display for DistinctOrderedFive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.higher(), self.lower())
    }
}

