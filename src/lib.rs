#![feature(const_fn)]

#[macro_use]
extern crate derive_more;

#[macro_use]
mod xenum;

pub mod chips;
mod game;

mod suit;
mod value;
//mod rank;
//mod cards;
//mod cardstats;


pub use suit::Suit;
pub use value::Value;
//pub use rank::Rank;
//pub use cards::{Card, Cards};
//pub use cardstats::CardStats;

