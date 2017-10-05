use std::fmt;

use sequential::{Sequential, LowBound};
use value::Value;
use value::Value::*;
use rank::intersect::{Intersect, IntersectOrd};
use rank::distinct_ordered::DistinctOrdered;


pub type DistinctOrderedTwo = DistinctOrdered<Value, Value>;


impl DistinctOrderedTwo {
    pub fn is_straight(&self) -> bool { self.higher() == self.lower().consequent().unwrap() }
}


impl LowBound for DistinctOrderedTwo {
    fn lowest() -> Self { Self::new(Three, Two) }
}


impl Intersect<DistinctOrderedTwo> for Value {
    fn intersects_with(&self, other: &DistinctOrderedTwo) -> bool { *self == other.higher() || *self == other.lower() }
}


impl IntersectOrd<DistinctOrderedTwo> for Value {
    fn all_greater_than(&self, other: &DistinctOrderedTwo) -> bool { *self > other.higher() }
}


impl fmt::Display for DistinctOrderedTwo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.higher(), self.lower())
    }
}

