mod distinct_two;
mod distinct_three;
mod distinct_five;
mod nothing;
mod pair;


use self::nothing::Nothing;
use self::pair::Pair;


pub enum RankType {
    Nothing,
    Pair,
    TwoPairs,
    Trips,
    Straight,
    Flush,
    FullHouse,
    Quads,
    StraightFlush
}


/// ```
/// use pokerbot::Value::*;
/// use pokerbot::Rank;
///
/// assert!( Rank::straight(Five) < Rank::quads(Three, Two) );
/// ```
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum Rank {
    Nothing(Nothing),
    Pair(Pair),
/*    TwoPairs(TwoPairs),
    Trips(Trips),
    Straight(Straight),
    Flush(Flush),
    FullHouse(FullHouse),
    Quads(Quads),
    StraightFlush(StraightFlush)
*/}


/*
impl Rank {
    pub fn new_pair(pair_value: Value, kickers: Kickers3) -> Self {
        assert!(!kickers.contains(pair_value));
        Rank::Pair { pair_value: pair_value, kickers: kickers }
    }
    pub fn new_two_pairs(higher_pair_value: Value, second_pair_value: Value, kicker: Value) -> Self {
        assert!(higher_pair_value > second_pair_value);
        assert!(higher_pair_value != kicker);
        assert!(second_pair_value != kicker);
        Rank::TwoPairs { higher_pair_value: higher_pair_value, second_pair_value: second_pair_value, kicker: kicker }
    }
    pub fn new_trips(trips_value: Value, kickers: Kickers2) -> Self {
        assert!(!kickers.contains(trips_value));
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

    pub fn next_rank(&self) -> Rank {
        match ref self {
        }
    }
}
*/
