use std::fmt;

use sequential::{Sequential, LowBound};
use value::Value;
use value::Value::*;
use rank::intersect::{Intersect, IntersectOrd};
use rank::distinct_ordered::DistinctOrdered;


pub type DistinctTwo = DistinctOrdered<Value, Value>;


impl DistinctTwo {
    pub fn is_straight(&self) -> bool { self.higher() == self.lower().consequent().unwrap() }
}


impl LowBound for DistinctTwo {
    fn lowest() -> Self { Self::new(Three, Two) }
}


impl Intersect<DistinctTwo> for Value {
    fn intersects_with(&self, other: &DistinctTwo) -> bool { *self == other.higher() || *self == other.lower() }
}


impl IntersectOrd<DistinctTwo> for Value {
    fn all_greater_than(&self, other: &DistinctTwo) -> bool { *self > other.higher() }
}


impl fmt::Display for DistinctTwo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.higher(), self.lower())
    }
}

