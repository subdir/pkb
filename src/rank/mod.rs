/// ```
/// use pokerbot::Value::*;
/// use pokerbot::Rank;
/// use pokerbot::rank::*;
///
/// assert!( Rank::straight(Straight::new(Five)) < Rank::quads(Quads::new(Three, Two)) );
/// ```

mod intersect;

mod distinct;
mod distinct_ordered;
mod distinct_ordered_two;
mod distinct_ordered_three;
mod distinct_ordered_five;

mod highcard;
mod pair;
mod two_pairs;
mod trips;
mod straight;
mod flush;
mod full_house;
mod quads;
mod straight_flush;


use std::fmt;

use sequential::{Sequential, LowBound};

pub use self::highcard::HighCard;
pub use self::pair::Pair;
pub use self::two_pairs::TwoPairs;
pub use self::trips::Trips;
pub use self::straight::Straight;
pub use self::flush::Flush;
pub use self::full_house::FullHouse;
pub use self::quads::Quads;
pub use self::straight_flush::StraightFlush;


#[derive(Debug)]
pub enum RankType {
    HighCard,
    Pair,
    TwoPairs,
    Trips,
    Straight,
    Flush,
    FullHouse,
    Quads,
    StraightFlush
}


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum Rank {
    HighCard(HighCard),
    Pair(Pair),
    TwoPairs(TwoPairs),
    Trips(Trips),
    Straight(Straight),
    Flush(Flush),
    FullHouse(FullHouse),
    Quads(Quads),
    StraightFlush(StraightFlush)
}


impl Rank {
    pub fn highcard(highcard: HighCard) -> Self { Rank::HighCard(highcard) }
    pub fn pair(pair: Pair) -> Self { Rank::Pair(pair) }
    pub fn two_pairs(two_pairs: TwoPairs) -> Self { Rank::TwoPairs(two_pairs) }
    pub fn trips(trips: Trips) -> Self { Rank::Trips(trips) }
    pub fn straight(straight: Straight) -> Self { Rank::Straight(straight) }
    pub fn flush(flush: Flush) -> Self { Rank::Flush(flush) }
    pub fn full_house(full_house: FullHouse) -> Self { Rank::FullHouse(full_house) }
    pub fn quads(quads: Quads) -> Self { Rank::Quads(quads) }
    pub fn straight_flush(straight_flush: StraightFlush) -> Self { Rank::StraightFlush(straight_flush) }

    pub fn sequence() -> impl Iterator<Item=Rank> {
        HighCard::lowest().sequence().map(|r| Rank::highcard(r))
        .chain(Pair::lowest().sequence().map(|r| Rank::pair(r)))
        .chain(TwoPairs::lowest().sequence().map(|r| Rank::two_pairs(r)))
        .chain(Trips::lowest().sequence().map(|r| Rank::trips(r)))
        .chain(Straight::lowest().sequence().map(|r| Rank::straight(r)))
        .chain(Flush::lowest().sequence().map(|r| Rank::flush(r)))
        .chain(FullHouse::lowest().sequence().map(|r| Rank::full_house(r)))
        .chain(Quads::lowest().sequence().map(|r| Rank::quads(r)))
        .chain(StraightFlush::lowest().sequence().map(|r| Rank::straight_flush(r)))
    }

    pub fn rank_type(&self) -> RankType {
        match *self {
            Rank::HighCard(_)       => RankType::HighCard,
            Rank::Pair(_)          => RankType::Pair,
            Rank::TwoPairs(_)      => RankType::TwoPairs,
            Rank::Trips(_)         => RankType::Trips,
            Rank::Straight(_)      => RankType::Straight,
            Rank::Flush(_)         => RankType::Flush,
            Rank::FullHouse(_)     => RankType::FullHouse,
            Rank::Quads(_)         => RankType::Quads,
            Rank::StraightFlush(_) => RankType::StraightFlush,
        }
    }
}


impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rank::HighCard(rank)       => rank.fmt(f),
            Rank::Pair(rank)          => rank.fmt(f),
            Rank::TwoPairs(rank)      => rank.fmt(f),
            Rank::Trips(rank)         => rank.fmt(f),
            Rank::Straight(rank)      => rank.fmt(f),
            Rank::Flush(rank)         => rank.fmt(f),
            Rank::FullHouse(rank)     => rank.fmt(f),
            Rank::Quads(rank)         => rank.fmt(f),
            Rank::StraightFlush(rank) => rank.fmt(f),
        }
    }
}

