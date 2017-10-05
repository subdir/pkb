mod distinct_two;
mod distinct_three;
mod distinct_five;
mod nothing;
mod pair;
mod two_pairs;
mod trips;
mod straight;
mod flush;

use std::fmt;

use sequential::Sequential;
use self::nothing::Nothing;
use self::pair::Pair;
use self::two_pairs::TwoPairs;
use self::trips::Trips;
use self::straight::Straight;
use self::flush::Flush;


#[derive(Debug)]
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
    TwoPairs(TwoPairs),
    Trips(Trips),
    Straight(Straight),
    Flush(Flush),
/*    FullHouse(FullHouse),
    Quads(Quads),
    StraightFlush(StraightFlush)
*/}


impl Rank {
    pub fn nothing(nothing: Nothing) -> Self { Rank::Nothing(nothing) }
    pub fn pair(pair: Pair) -> Self { Rank::Pair(pair) }
    pub fn two_pairs(two_pairs: TwoPairs) -> Self { Rank::TwoPairs(two_pairs) }
    pub fn trips(trips: Trips) -> Self { Rank::Trips(trips) }
    pub fn straight(straight: Straight) -> Self { Rank::Straight(straight) }
    pub fn flush(flush: Flush) -> Self { Rank::Flush(flush) }

    pub fn sequence() -> impl Iterator<Item=Rank> {
        Nothing::lowest().sequence().map(|r| Rank::nothing(r))
        .chain(Pair::lowest().sequence().map(|r| Rank::pair(r)))
        .chain(TwoPairs::lowest().sequence().map(|r| Rank::two_pairs(r)))
        .chain(Trips::lowest().sequence().map(|r| Rank::trips(r)))
        .chain(Straight::lowest().sequence().map(|r| Rank::straight(r)))
        .chain(Flush::lowest().sequence().map(|r| Rank::flush(r)))
    }

    pub fn rank_type(&self) -> RankType {
        match *self {
            Rank::Nothing(rank)  => RankType::Nothing,
            Rank::Pair(rank)     => RankType::Pair,
            Rank::TwoPairs(rank) => RankType::TwoPairs,
            Rank::Trips(rank)    => RankType::Trips,
            Rank::Straight(rank) => RankType::Straight,
            Rank::Flush(rank)    => RankType::Flush,
        }
    }
}


impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rank::Nothing(rank)  => write!(f, "{} {:?}", rank, self.rank_type()),
            Rank::Pair(rank)     => write!(f, "{} {:?}", rank, self.rank_type()),
            Rank::TwoPairs(rank) => write!(f, "{} {:?}", rank, self.rank_type()),
            Rank::Trips(rank)    => write!(f, "{} {:?}", rank, self.rank_type()),
            Rank::Straight(rank) => write!(f, "{} {:?}", rank, self.rank_type()),
            Rank::Flush(rank)    => write!(f, "{} {:?}", rank, self.rank_type()),
        }
    }
}


/*
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
