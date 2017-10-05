use std::fmt;

use sequential::Sequential;
use value::Value;
use rank::distinct_two::DistinctTwo;


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Trips {
    trips_value: Value,
    kickers: DistinctTwo,
}


impl Trips {
    pub fn new(trips_value: Value, kickers: DistinctTwo) -> Self {
        assert!(!kickers.contains(trips_value));
        Self { trips_value: trips_value, kickers: kickers }
    }

    pub fn lowest() -> Self {
        Self::lowest_for(Value::Two)
    }

    pub fn lowest_for(trips_value: Value) -> Self {
        Self {
            trips_value: trips_value,
            kickers: DistinctTwo::lowest().skip_value(trips_value).unwrap()
        }
    }
}


impl Sequential for Trips {
    fn consequent(&self) -> Option<Self> {
        match self.kickers.consequent().and_then(|k| k.skip_value(self.trips_value)) {
            Some(next_kickers) => Some(Self::new(self.trips_value, next_kickers)),
            None => match self.trips_value.consequent() {
                Some(next_trips_value) => Some(Self::lowest_for(next_trips_value)),
                None => None
            }
        }
    }
}


impl fmt::Display for Trips {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}{}", self.trips_value, self.trips_value, self.trips_value, self.kickers)
    }
}

