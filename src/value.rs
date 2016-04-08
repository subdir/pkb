use std::fmt;

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
    Ace,
}

static all_values: [Value; 13] = [
    Value::Two,
    Value::Three,
    Value::Four,
    Value::Five,
    Value::Six,
    Value::Seven,
    Value::Eight,
    Value::Nine,
    Value::Ten,
    Value::Jack,
    Value::Queen,
    Value::King,
    Value::Ace,
];

impl Value {
    pub fn all() -> &'static[Value] {
        &all_values
    }

    pub fn id(&self) -> u8 {
        *self as u8
    }

    pub fn from_id(id: u8) -> Self {
        all_values[id as usize]
    }

    pub fn to_char(&self) -> char {
        match *self {
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
            Value::Ace   => 'A',
        }
    }

    pub fn from_char(c: char) -> Self {
        match c {
            '2' => Value::Two,
            '3' => Value::Three,
            '4' => Value::Four,
            '5' => Value::Five,
            '6' => Value::Six,
            '7' => Value::Seven,
            '8' => Value::Eight,
            '9' => Value::Nine,
            'T' => Value::Ten,
            'J' => Value::Jack,
            'Q' => Value::Queen,
            'K' => Value::King,
            'A' => Value::Ace,
            _ => panic!(format!("Bad value char {:?}", c)),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_char())
    }
}
