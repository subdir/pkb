extern crate pokerbot;

use pokerbot::Suit;
use pokerbot::Cards;

fn main() {
    println!("XXXXXX {:?}", Cards::from_str("AS:5H:2D:3D:4D"));
    println!("{:?}", Suit::variants_num());
}
