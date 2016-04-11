xenum!{
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}}

impl Value {
    inverted_match_constructor! {
    pub fn from_char(char) -> Self {}
    pub fn to_char(self) -> char {
        match self {
            Value::Two   => '2',
            Value::Three => '3',
            Value::Four  => '4',
            Value::Five  => '5',
            Value::Six   => '6',
            Value::Seven => '7',
            Value::Eight => '8',
            Value::Nine  => '9',
            Value::Ten   => 'T',
            Value::Jack  => 'J',
            Value::Queen => 'Q',
            Value::King  => 'K',
            Value::Ace   => 'A'
        }
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.to_char())
    }
}
