use std::fmt;

use sequential::{Sequential, LowBound};
use value::Value;


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Straight {
    higher: Value,
}


impl Straight {
    pub fn new(higher: Value) -> Self {
        assert!(higher >= Value::Five);
        Self { higher: higher }
    }

    pub fn ranks_count() -> usize { 10 }
}


impl LowBound for Straight {
    fn lowest() -> Self { Self { higher: Value::Five } }
}


impl Sequential for Straight {
    fn consequent(&self) -> Option<Self> {
        self.higher.consequent().map(|h| Self::new(h))
    }
}


impl fmt::Display for Straight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "{}{}{}{}{}",
            self.higher,
            self.higher.prev().unwrap(),
            self.higher.prev().unwrap().prev().unwrap(),
            self.higher.prev().unwrap().prev().unwrap().prev().unwrap(),
            self.higher.prev().unwrap().prev().unwrap().prev().unwrap().prev().unwrap_or(Value::Ace),
        )
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ranks_count() {
        assert_eq!(Straight::ranks_count(), Straight::lowest().sequence().count());
    }
}
