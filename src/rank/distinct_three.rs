use std::fmt;

use sequential::Sequential;
use value::Value;
use value::Value::*;
use rank::distinct_two::DistinctTwo;


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct DistinctThree {
    higher: Value,
    lower_two: DistinctTwo,
}


impl DistinctThree {
    pub fn new(higher: Value, middle: Value, lower: Value) -> Self {
        assert!(higher > middle);
        Self {
            higher: higher,
            lower_two: DistinctTwo::new(middle, lower)
        }
    }

    pub fn lowest() -> Self {
        Self::new(Four, Three, Two)
    }

    pub fn higher(&self) -> Value { self.higher }
    pub fn middle(&self) -> Value { self.lower_two.higher() }
    pub fn lower(&self) -> Value { self.lower_two.lower() }

    pub fn contains(&self, value: Value) -> bool {
        value == self.higher
        || self.lower_two.contains(value)
    }

    pub fn is_straight(&self) -> bool {
        self.higher == self.lower_two.higher().consequent().unwrap()
        && self.lower_two.is_straight()
    }

    pub fn skip_value(&self, value: Value) -> Option<Self> {
        self
        .sequence()
        .skip_while(|d| d.contains(value))
        .next()
    }
}


impl Sequential for DistinctThree {
    fn consequent(&self) -> Option<Self> {
        let next_lower_two = self.lower_two.consequent().unwrap();

        if self.higher > next_lower_two.higher() {
            Some(Self{ higher: self.higher, lower_two: next_lower_two })
        } else {
            match self.higher.consequent() {
                None => None,
                Some(next_higher) => Some(Self{
                    higher: next_higher,
                    lower_two: DistinctTwo::lowest()
                })
            }
        }
    }
}


impl fmt::Display for DistinctThree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.higher, self.lower_two)
    }
}

