#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
#[derive(Add, Sub)]
pub struct Chips(u64);

impl Chips {
    pub fn new(amount: u64) -> Chips { Chips(amount) }
}

