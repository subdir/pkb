use std::fmt;

use sequential::Sequential;
use value::Value;
use value::Value::*;
use rank::distinct_two::DistinctTwo;
use rank::distinct_three::DistinctThree;


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct DistinctFive {
    higher_two: DistinctTwo,
    lower_three: DistinctThree,
}


impl DistinctFive {
    pub fn lowest() -> Self {
        DistinctFive {
            higher_two: DistinctTwo::new(Six, Five),
            lower_three: DistinctThree::new(Four, Three, Two)
        }
    }

    pub fn contains(&self, value: Value) -> bool {
        self.higher_two.contains(value) || self.lower_three.contains(value)
    }

    pub fn is_straight(&self) -> bool {
        self.higher_two.is_straight()
        && self.lower_three.is_straight()
        && self.higher_two.lower() == self.lower_three.higher().consequent().unwrap()
    }

    pub fn skip_straight(&self) -> Option<Self> {
        self
        .sequence()
        .skip_while(|f| f.is_straight())
        .next()
    }
}


impl Sequential for DistinctFive {
    fn consequent(&self) -> Option<Self> {
        let next_lower_three = self.lower_three.consequent().unwrap();

        if self.higher_two.lower() > next_lower_three.higher() {
            Some(Self{
                higher_two: self.higher_two,
                lower_three: next_lower_three
            })
        } else {
            match self.higher_two.consequent() {
                None => None,
                Some(next_higher_two) => Some(Self{
                    higher_two: next_higher_two,
                    lower_three: DistinctThree::lowest()
                })
            }
        }
    }
}


impl fmt::Display for DistinctFive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.higher_two, self.lower_three)
    }
}

