use std::fmt;

use sequential::Sequential;
use value::Value;
use rank::distinct_two::DistinctTwo;


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct TwoPairs {
    pairs: DistinctTwo,
    kicker: Value,
}


impl TwoPairs {
    pub fn new(pairs: DistinctTwo, kicker: Value) -> Self {
        assert!(!pairs.contains(kicker));
        Self { pairs: pairs, kicker: kicker }
    }

    pub fn lowest() -> Self {
        Self::lowest_for(DistinctTwo::lowest())
    }

    pub fn lowest_for(pairs: DistinctTwo) -> Self {
        Self {
            pairs: pairs,
            kicker: pairs.skip_contained_values(Value::lowest()).unwrap()
        }
    }
}


impl Sequential for TwoPairs {
    fn consequent(&self) -> Option<Self> {
        match self.kicker.consequent().and_then(|k| self.pairs.skip_contained_values(k)) {
            Some(next_kicker) => Some(Self::new(self.pairs, next_kicker)),
            None => match self.pairs.consequent() {
                Some(next_pairs) => Some(Self::lowest_for(next_pairs)),
                None => None
            }
        }
    }
}


impl fmt::Display for TwoPairs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "{}{}{}{}{}",
            self.pairs.higher(),
            self.pairs.higher(),
            self.pairs.lower(),
            self.pairs.lower(),
            self.kicker
        )
    }
}

