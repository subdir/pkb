#![feature(const_fn)]
#![feature(conservative_impl_trait)]

#[macro_use]
extern crate derive_more;
extern crate itertools;
extern crate rand;

#[macro_use]
pub mod util;
pub use util::ordered_iter;
pub use util::sequential;
pub use util::xenum;

pub mod chips;
pub mod value;
pub mod suit;
pub mod card;
pub mod deck;
pub mod shuffle;
pub mod rank;
pub mod ranking;

pub mod game;

pub use value::Value;
pub use suit::Suit;
pub use card::Card;
pub use rank::Rank;

