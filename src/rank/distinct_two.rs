use std::fmt;

use sequential::Sequential;
use value::Value;
use value::Value::*;


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct DistinctTwo {
    higher: Value,
    lower: Value,
}


impl DistinctTwo {
    pub fn new(higher: Value, lower: Value) -> Self {
        assert!(higher > lower);
        Self{ higher: higher, lower: lower }
    }

    pub fn lowest() -> Self {
        Self::new(Three, Two)
    }

    pub fn higher(&self) -> Value { self.higher }
    pub fn lower(&self) -> Value { self.lower }
    pub fn contains(&self, value: Value) -> bool { value == self.higher || value == self.lower }
    pub fn is_straight(&self) -> bool { self.higher == self.lower.consequent().unwrap() }

    pub fn skip_contained_values(&self, value: Value) -> Option<Value> {
        value
        .sequence()
        .skip_while(|v| self.contains(*v))
        .next()
    }

    pub fn skip_le(&self, value: Value) -> Option<DistinctTwo> {
        self
        .sequence()
        .skip_while(|d| d.lower() <= value)
        .next()
    }

    pub fn skip_value(&self, value: Value) -> Option<Self> {
        self
        .sequence()
        .skip_while(|d| d.contains(value))
        .next()
    }
}


impl Sequential for DistinctTwo {
    fn consequent(&self) -> Option<Self> {
        let next_lower = self.lower.consequent().unwrap();

        if self.higher > next_lower {
            Some(Self::new(self.higher, next_lower))
        } else {
            match self.higher.consequent() {
                None => None,
                Some(next_higher) => Some(Self::new(next_higher, Value::lowest()))
            }
        }
    }
}


impl fmt::Display for DistinctTwo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.higher, self.lower)
    }
}

