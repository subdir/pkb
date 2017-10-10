use std::fmt;

use sequential::{Sequential, LowBound};
use value::Value;
use rank::intersect::Intersect;
use rank::distinct::Distinct;
use rank::distinct_ordered_three::DistinctOrderedThree;
use rank::combinations_count;


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Pair {
    pair: Distinct<Value, DistinctOrderedThree>
}


impl Pair {
    pub fn new(pair_value: Value, kickers: DistinctOrderedThree) -> Self {
        Self { pair: Distinct::new(pair_value, kickers) }
    }

    pub fn lowest_for(pair_value: Value) -> Self {
        Self { pair: Distinct::new(
            pair_value,
            DistinctOrderedThree::lowest().skip_intersecting(&pair_value).unwrap()
        )}
    }

    pub fn pair_value(&self) -> Value { self.pair.primary() }
    pub fn kickers(&self) -> DistinctOrderedThree { self.pair.secondary() }

    pub fn ranks_count() -> usize {
        Value::VARIANTS_NUM * combinations_count(3, Value::VARIANTS_NUM - 1)
    }
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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ranks_count() {
        assert_eq!(Pair::ranks_count(), Pair::lowest().sequence().count());
    }
}
