use std::fmt;

use sequential::{Sequential, LowBound};
use value::Value;
use rank::intersect::Intersect;
use rank::distinct::Distinct;
use rank::distinct_two::DistinctTwo;


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct TwoPairs {
    pairs: Distinct<DistinctTwo, Value>
}


impl TwoPairs {
    pub fn new(pairs: DistinctTwo, kicker: Value) -> Self {
        Self { pairs: Distinct::new(pairs, kicker) }
    }

    pub fn lowest_for(pairs: DistinctTwo) -> Self {
        Self { pairs: Distinct::new(
            pairs,
            Value::lowest().skip_intersecting(&pairs).unwrap()
        )}
    }

    fn higher_pair_value(&self) -> Value { self.pairs.primary().higher() }
    fn lower_pair_value(&self) -> Value { self.pairs.primary().lower() }
    fn kicker(&self) -> Value { self.pairs.secondary() }
}


impl LowBound for TwoPairs {
    fn lowest() -> Self { Self::lowest_for(DistinctTwo::lowest()) }
}


impl Sequential for TwoPairs {
    fn consequent(&self) -> Option<Self> {
        self.pairs.consequent().map(|p| Self { pairs: p })
    }
}


impl fmt::Display for TwoPairs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "{}{}{}{}{}",
            self.higher_pair_value(),
            self.higher_pair_value(),
            self.lower_pair_value(),
            self.lower_pair_value(),
            self.kicker()
        )
    }
}

