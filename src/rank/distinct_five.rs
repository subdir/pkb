use std::fmt;

use sequential::Sequential;
use value::Value::*;
use rank::distinct_ordered::DistinctOrdered;
use rank::distinct_two::DistinctTwo;
use rank::distinct_three::DistinctThree;


pub type DistinctFive = DistinctOrdered<DistinctTwo, DistinctThree>;


impl DistinctFive {
    pub fn lowest() -> Self {
        Self::new(
            DistinctTwo::new(Six, Five),
            DistinctThree::new_three(Four, Three, Two)
        )
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


impl fmt::Display for DistinctFive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.higher(), self.lower())
    }
}

