use std::fmt;

use sequential::{Sequential, LowBound};
use value::Value;
use value::Value::*;
use rank::intersect::{Intersect, IntersectOrd};
use rank::distinct_ordered::DistinctOrdered;
use rank::distinct_two::DistinctTwo;


pub type DistinctThree = DistinctOrdered<Value, DistinctTwo>;


impl DistinctThree {
    pub fn new_three(higher: Value, middle: Value, lower: Value) -> Self {
        Self::new(higher, DistinctTwo::new(middle, lower))
    }

/*    pub fn higher(&self) -> Value { self.values.higher() }
    pub fn middle(&self) -> Value { self.values.lower().higher() }
    pub fn lower(&self) -> Value { self.values.lower().lower() }
*/

    pub fn is_straight(&self) -> bool {
        self.lower().is_straight()
        && self.higher() == self.lower().higher().consequent().unwrap()
    }
}


impl LowBound for DistinctThree {
    fn lowest() -> Self { Self::new_three(Four, Three, Two) }
}


impl Intersect<DistinctThree> for Value {
    fn intersects_with(&self, other: &DistinctThree) -> bool {
        *self == other.higher() || self.intersects_with(&other.lower())
    }
}


impl IntersectOrd<DistinctThree> for DistinctTwo {
    fn all_greater_than(&self, other: &DistinctThree) -> bool { self.lower() > other.higher() }
}


impl fmt::Display for DistinctThree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.higher(), self.lower())
    }
}

