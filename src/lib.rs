#![feature(const_fn)]
#![feature(conservative_impl_trait)]

#[macro_use]
extern crate derive_more;
extern crate itertools;
extern crate rand;

#[macro_use]
mod xenum;
mod sequential;
mod sorted_iter;

pub mod suit;
pub mod value;
pub mod card;
pub mod deck;
pub mod shuffle;

pub mod game;

pub mod chips;
pub mod rank;

pub use suit::Suit;
pub use value::Value;
pub use card::Card;
pub use rank::Rank;

