use std::fmt;

use sequential::{Sequential, LowBound};
use value::Value;
use rank::intersect::Intersect;
use rank::distinct::Distinct;
use rank::distinct_three::DistinctThree;


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Pair {
    pair: Distinct<Value, DistinctThree>
}


impl Pair {
    pub fn new(pair_value: Value, kickers: DistinctThree) -> Self {
        Self { pair: Distinct::new(pair_value, kickers) }
    }

    pub fn lowest_for(pair_value: Value) -> Self {
        Self { pair: Distinct::new(
            pair_value,
            DistinctThree::lowest().skip_intersecting(&pair_value).unwrap()
        )}
    }

    pub fn pair_value(&self) -> Value { self.pair.primary() }
    pub fn kickers(&self) -> DistinctThree { self.pair.secondary() }
}


impl LowBound for Pair {
    fn lowest() -> Self { Self::lowest_for(Value::Two) }
}


impl Sequential for Pair {
    fn consequent(&self) -> Option<Self> {
        self.pair.consequent().map(|c| Self { pair: c })
    }
}


impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.pair_value(), self.pair_value(), self.kickers())
    }
}

