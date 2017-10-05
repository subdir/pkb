use std::fmt;

use sequential::Sequential;
use value::Value;
use rank::nothing::Nothing;


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Flush {
    values: Nothing,
}


impl Flush {
    pub fn new(values: Nothing) -> Self {
        Self { values: values }
    }

    pub fn lowest() -> Self {
        Self { values: Nothing::lowest() }
    }
}


impl Sequential for Flush {
    fn consequent(&self) -> Option<Self> {
        self.values.consequent().map(|v| Self::new(v))
    }
}


impl fmt::Display for Flush {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}*", self.values)
    }
}

