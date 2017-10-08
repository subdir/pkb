pub trait Countable {
    fn from_serial(serial: usize) -> Self;
    fn to_serial(&self) -> usize;
}

