extern crate pokerbot;

use pokerbot::Suit;

fn main() {
    let numbers = 0..5;
    println!("{:?}", numbers.collect::<Vec<i32>>());
    /*
    println!("{:?}", Suit::Hearts.id());
    println!("{:?}", Suit::Spades.id());
    println!("{:?}", Suit::from_id(0));
    println!("{:?}", Suit::from_id(1));
    println!("{:?}", Suit::from_id(2));
    println!("{:?}", Suit::from_id(3));
    println!("{:?}", Suit::from_id(4));
*/}
