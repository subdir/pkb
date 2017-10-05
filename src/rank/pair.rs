use sequential::Sequential;
use value::Value;
use rank::distinct_three::DistinctThree;


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Pair {
    pair_value: Value,
    kickers: DistinctThree,
}


impl Pair {
    fn new(pair_value: Value, kickers: DistinctThree) -> Self {
        assert!(!kickers.contains(pair_value));
        Self { pair_value: pair_value, kickers: kickers }
    }

    fn lowest() -> Self {
        Self::lowest_for(Value::Two)
    }

    fn lowest_for(pair_value: Value) -> Self {
        Self {
            pair_value: pair_value,
            kickers: DistinctThree::lowest().skip_value(pair_value).unwrap()
        }
    }
}


impl Sequential for Pair {
    fn consequent(&self) -> Option<Self> {
        match self.kickers.consequent().and_then(|k| k.skip_value(self.pair_value)) {
            Some(next_kickers) => Some(Self::new(self.pair_value, next_kickers)),
            None => match self.pair_value.consequent() {
                Some(next_pair_value) => Some(Self::lowest_for(next_pair_value)),
                None => None
            }
        }
    }
}

