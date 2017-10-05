use std::fmt;

use sequential::{Sequential, LowBound};
use value::Value;
use rank::intersect::Intersect;
use rank::distinct::Distinct;
use rank::distinct_ordered_two::DistinctOrderedTwo;


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Trips {
    trips: Distinct<Value, DistinctOrderedTwo>
}


impl Trips {
    pub fn new(trips_value: Value, kickers: DistinctOrderedTwo) -> Self {
        Self { trips: Distinct::new(trips_value, kickers) }
    }

    pub fn lowest_for(trips_value: Value) -> Self {
        Self { trips: Distinct::new(
            trips_value,
            DistinctOrderedTwo::lowest().skip_intersecting(&trips_value).unwrap()
        )}
    }

    pub fn trips_value(&self) -> Value { self.trips.primary() }
    pub fn kickers(&self) -> DistinctOrderedTwo { self.trips.secondary() }
}


impl LowBound for Trips {
    fn lowest() -> Self { Self::lowest_for(Value::Two) }
}


impl Sequential for Trips {
    fn consequent(&self) -> Option<Self> {
        self.trips.consequent().map(|t| Self { trips: t } )
    }
}


impl fmt::Display for Trips {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}{}", self.trips_value(), self.trips_value(), self.trips_value(), self.kickers())
    }
}

