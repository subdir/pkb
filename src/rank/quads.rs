use std::fmt;

use sequential::{Sequential, LowBound};
use value::Value;
use rank::distinct::Distinct;


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Quads {
    quads: Distinct<Value, Value>
}


impl Quads {
    pub fn new(quads_value: Value, kicker: Value) -> Self {
        Self { quads: Distinct::new(quads_value, kicker) }
    }

    pub fn quads_value(&self) -> Value { self.quads.primary() }
    pub fn kicker(&self) -> Value { self.quads.secondary() }
}


impl LowBound for Quads {
    fn lowest() -> Self { Self::new(Value::Two, Value::Three) }
}


impl Sequential for Quads {
    fn consequent(&self) -> Option<Self> {
        self.quads.consequent().map(|h| Self { quads: h })
    }
}


impl fmt::Display for Quads {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "{}{}{}{}{}",
            self.quads_value(),
            self.quads_value(),
            self.quads_value(),
            self.quads_value(),
            self.kicker(),
        )
    }
}

