xenum! {
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs
}}


impl Suit {
    inverted_match_constructor! {
        pub fn from_char(char) -> Self {}
        pub fn to_char(self) -> char {
            match self {
                Suit::Spades => 'S',
                Suit::Hearts => 'H',
                Suit::Diamonds => 'D',
                Suit::Clubs => 'C'
            }
        }
    }
}


impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.to_char())
    }
}

