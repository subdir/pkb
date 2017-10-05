use std::fmt;

use sequential::Sequential;
use value::Value;


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct FullHouse {
    three_card_value: Value,
    two_card_value: Value,
}


impl FullHouse {
    pub fn new(three_card_value: Value, two_card_value: Value) -> Self {
        assert!(three_card_value != two_card_value);
        Self {
            three_card_value: three_card_value,
            two_card_value: two_card_value
        }
    }

    pub fn lowest() -> Self {
        Self {
            three_card_value: Value::Two,
            two_card_value: Value::Three
        }
    }
}


impl Sequential for FullHouse {
    fn consequent(&self) -> Option<Self> {
        match self.two_card_value.consequent().and_then(|k| k.skip_value(self.three_card_value)) {
            Some(next_two_card_value) => Some(Self::new(self.three_card_value, next_two_card_value)),
            None => match self.three_card_value.consequent() {
                Some(next_three_card_value) => Some(Self::new(next_three_card_value, Value::lowest())),
                None => None
            }
        }
    }
}


impl fmt::Display for FullHouse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "{}{}{}{}{}",
            self.three_card_value,
            self.three_card_value,
            self.three_card_value,
            self.two_card_value,
            self.two_card_value,
        )
    }
}

