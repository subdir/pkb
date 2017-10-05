#![feature(const_fn)]
#![feature(conservative_impl_trait)]

#[macro_use]
extern crate derive_more;

#[macro_use]
mod xenum;

mod sequential;

mod game;

mod suit;
mod value;

pub mod chips;
pub mod rank;

pub use suit::Suit;
pub use value::Value;
pub use rank::Rank;

