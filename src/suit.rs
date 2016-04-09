use std::fmt;

xenum! {
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs
}
}

static ALL_SUITS: [Suit; 4] = [
    Suit::Spades,
    Suit::Hearts,
    Suit::Diamonds,
    Suit::Clubs,
];

impl Suit {
    pub fn all() -> &'static[Suit] {
        &ALL_SUITS
    }

    pub fn to_char(&self) -> char {
        match *self {
            Suit::Spades   => 'S',
            Suit::Hearts   => 'H',
            Suit::Diamonds => 'D',
            Suit::Clubs    => 'C',
        }
    }

    pub fn from_char(c: char) -> Self {
        match c {
            'S' => Suit::Spades,
            'H' => Suit::Hearts,
            'D' => Suit::Diamonds,
            'C' => Suit::Clubs,
            _ => panic!(format!("Bad suit char {:?}", c)),
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_char())
    }
}

#[test]
fn test() {
    assert!(Suit::from_u16(0).unwrap() == Suit::Spades);
    assert!(Suit::from_u8(1).unwrap() == Suit::Hearts);
}
