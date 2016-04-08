use std::fmt;

#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

static all_suits: [Suit; 4] = [
    Suit::Spades,
    Suit::Hearts,
    Suit::Diamonds,
    Suit::Clubs,
];

impl Suit {
    pub fn all() -> &'static[Suit] {
        &all_suits
    }

    pub fn id(&self) -> u8 {
        *self as u8
    }

    pub fn from_id(id: u8) -> Self {
        all_suits[id as usize]
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

