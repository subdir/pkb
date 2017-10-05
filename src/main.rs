extern crate pokerbot;

use pokerbot::Rank;


fn main() {
    for rank in Rank::sequence() {
        println!("{} {:?}", rank, rank.rank_type());
    }

//    println!("{}", (&mut g).next().unwrap().next().unwrap());

//    println!("XXXXXX {:?}", Cards::from_str("AS:5H:2D:3D:4D"));
//    println!("{:?}", Suit::variants_num());
}

