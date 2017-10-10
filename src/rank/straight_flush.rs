use std::fmt;

use sequential::{Sequential, LowBound};
use rank::straight::Straight;


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct StraightFlush {
    straight: Straight,
}


impl StraightFlush {
    pub fn new(straight: Straight) -> Self { Self { straight: straight } }

    pub fn ranks_count() -> usize {
        Straight::ranks_count()
    }
}


impl LowBound for StraightFlush {
    fn lowest() -> Self { Self { straight: Straight::lowest() } }
}


impl Sequential for StraightFlush {
    fn consequent(&self) -> Option<Self> {
        self.straight.consequent().map(|v| Self::new(v))
    }
}


impl fmt::Display for StraightFlush {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}*", self.straight)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ranks_count() {
        assert_eq!(StraightFlush::ranks_count(), StraightFlush::lowest().sequence().count());
    }
}
