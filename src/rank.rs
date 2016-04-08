use value::Value;

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Kickers2(Value, Value);
impl Kickers2 {
    fn new(kicker1: Value, kicker2: Value) -> Self {
        // kickers with the same value are impossible
        assert!(kicker1 > kicker2);
        Kickers2(kicker1, kicker2)
    }
    fn has_value(&self, value: Value) -> bool {
        value == self.0 || value == self.1
    }
}


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Kickers3(Value, Value, Value);
impl Kickers3 {
    fn new(kicker1: Value, kicker2: Value, kicker3: Value) -> Self {
        // kickers with the same value are impossible
        assert!(kicker1 > kicker2 && kicker2 > kicker3);
        Kickers3(kicker1, kicker2, kicker3)
    }
    fn has_value(&self, value: Value) -> bool {
        value == self.0 || value == self.1 || value == self.2
    }
}


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Kickers5(Value, Value, Value, Value, Value);
impl Kickers5 {
    fn new(kicker1: Value, kicker2: Value, kicker3: Value, kicker4: Value, kicker5: Value) -> Self {
        // kickers with the same value are impossible
        assert!(kicker1 > kicker2 && kicker2 > kicker3 && kicker3 > kicker4 && kicker4 > kicker5);
        Kickers5(kicker1, kicker2, kicker3, kicker4, kicker5)
    }
}


/// ```
/// use pokerbot::{Value, Rank};
/// assert!( Rank::new_straight(Value::Five) < Rank::new_quads(Value::Three, Value::Two) );
/// ```
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum Rank {
    Nothing {
        values: Kickers5,
    },
    Pair {
        pair_value: Value,
        kickers: Kickers3,
    },
    TwoPairs {
        higher_pair_value: Value,
        second_pair_value: Value,
        kicker: Value,
    },
    Trips {
        trips_value: Value,
        kickers: Kickers2,
    },
    Straight {
        highest_value: Value,
    },
    Flush {
        values: Kickers5,
    },
    FullHouse {
        three_card_value: Value,
        two_card_value: Value,
    },
    Quads {
        value: Value,
        kicker: Value,
    },
    StraightFlush {
        highest_value: Value,
    }
}


impl Rank {
    pub fn new_nothing(values: Kickers5) -> Self {
        Rank::Nothing { values: values }
    }
    pub fn new_pair(pair_value: Value, kickers: Kickers3) -> Self {
        assert!(!kickers.has_value(pair_value));
        Rank::Pair { pair_value: pair_value, kickers: kickers }
    }
    pub fn new_two_pairs(higher_pair_value: Value, second_pair_value: Value, kicker: Value) -> Self {
        assert!(higher_pair_value != second_pair_value);
        assert!(higher_pair_value != kicker);
        assert!(second_pair_value != kicker);
        Rank::TwoPairs { higher_pair_value: higher_pair_value, second_pair_value: second_pair_value, kicker: kicker }
    } 
    pub fn new_trips(trips_value: Value, kickers: Kickers2) -> Self {
        assert!(!kickers.has_value(trips_value));
        Rank::Trips { trips_value: trips_value, kickers: kickers }
    }
    pub fn new_straight(highest_value: Value) -> Self {
        assert!(highest_value >= Value::Five);
        Rank::Straight { highest_value: highest_value }
    }
    pub fn new_flush(values: Kickers5) -> Self {
        Rank::Flush { values: values }
    }
    pub fn new_full_house(three_card_value: Value, two_card_value: Value) -> Self {
        assert!(three_card_value != two_card_value);
        Rank::FullHouse { three_card_value: three_card_value, two_card_value: two_card_value }
    }
    pub fn new_quads(value: Value, kicker: Value) -> Self {
        assert!(value != kicker);
        Rank::Quads { value: value, kicker: kicker }
    }
    pub fn new_straight_flush(highest_value: Value) -> Self {
        assert!(highest_value >= Value::Five);
        Rank::StraightFlush { highest_value: highest_value }
    }
}

