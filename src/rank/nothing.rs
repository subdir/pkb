use std::fmt;

use sequential::{Sequential, LowBound};
use rank::distinct_ordered_five::DistinctOrderedFive;


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Nothing {
    values: DistinctOrderedFive,
}


impl LowBound for Nothing {
    fn lowest() -> Self {
        Self{ values: DistinctOrderedFive::lowest().skip_straight().unwrap() }
    }
}


impl Sequential for Nothing {
    fn consequent(&self) -> Option<Self> {
        self.values
        .consequent()
        .and_then(|c| c.skip_straight())
        .map(|c| Self { values: c })
    }
}


impl fmt::Display for Nothing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.values)
    }
}

