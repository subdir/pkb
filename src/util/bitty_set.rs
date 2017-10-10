use std::fmt;
use countable::Countable;


#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Hash)]
pub struct BittySet<T: Countable, B: Bits + Clone> {
    bits: B,
    type_holder: Option<T>
}


impl<T: Countable, B: Bits + Clone> BittySet<T, B> {
    pub fn new() -> Self { Self { bits: B::zero(), type_holder: None } }

    pub fn add(&mut self, val: &T) {
        self.bits.set(val.to_serial())
    }

    pub fn is_empty(&self) -> bool {
        self.bits == B::zero()
    }

    pub fn contains(&mut self, val: &T) -> bool {
        self.bits.get(val.to_serial())
    }

    pub fn desc_iter(&self) -> impl Iterator<Item=T> {
        self.bits.clone().set_bits_desc_iter().map(|b| T::from_serial(b))
    }
}


impl<T: Countable + fmt::Debug, B: Bits + Clone> fmt::Debug for BittySet<T, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "BittySet{{ {:?} }}", self.desc_iter().collect::<Vec<T>>())
    }
}


pub trait Bits: Sized + PartialEq {
    fn zero() -> Self;
    fn size() -> usize;
    fn get(&mut self, bit: usize) -> bool;
    fn set(&mut self, bit: usize);
    fn set_bits_desc_iter(self) -> SetBitsDescIter<Self> {
        SetBitsDescIter { bits: self, index: Self::size() - 1 }
    }
}


impl Bits for u8 {
    fn zero() -> Self { 0 }
    fn size() -> usize { 8 }

    fn get(&mut self, bit: usize) -> bool {
        assert!(bit < Self::size());
        (*self & (1 << bit)) != 0
    }

    fn set(&mut self, bit: usize) {
        assert!(bit < Self::size());
        *self |= 1 << bit;
    }
}


impl Bits for u64 {
    fn zero() -> Self { 0 }
    fn size() -> usize { 64 }

    fn get(&mut self, bit: usize) -> bool {
        assert!(bit < Self::size());
        (*self & (1 << bit)) != 0
    }

    fn set(&mut self, bit: usize) {
        assert!(bit < Self::size());
        *self |= 1 << bit;
    }
}


pub struct SetBitsDescIter<B: Bits> {
    bits: B,
    index: usize
}


impl<B: Bits> Iterator for SetBitsDescIter<B> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < B::size() {
            let index = self.index;
            let bit_is_set = self.bits.get(index);

            if self.index == 0 {
                self.index = B::size();
            } else {
                self.index -= 1;
            }

            if bit_is_set {
                return Some(index)
            }
        }
        None
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use self::HexLetter::*;

    #[test]
    fn test_bits() {
        let mut bits = 0u64;
        bits.set(2);
        assert_eq!(bits, 4);
        bits.set(4);
        assert_eq!(vec!(4, 2), bits.set_bits_desc_iter().collect::<Vec<usize>>());
    }

    #[test]
    fn test_bitty_set() {
        let mut set = BittySet::<HexLetter, u8>::new();
        assert!(!set.contains(&A));
        set.add(&A);
        assert!(set.contains(&A));
        set.add(&E);
        set.add(&F);
        assert_eq!(vec!(F, E, A), set.desc_iter().collect::<Vec<HexLetter>>());
    }

    #[derive(Debug)]
    #[derive(Eq, PartialEq)]
    enum HexLetter {
        A,
        B,
        C,
        D,
        E,
        F
    }

    impl Countable for HexLetter {
        inverted_match_constructor! {
            fn from_serial(usize) -> Self { /* generated by inverted_match_constructor */ }
            fn to_serial(&self) -> usize {
                match *self {
                    A => 0,
                    B => 1,
                    C => 2,
                    D => 3,
                    E => 4,
                    F => 5
                }
            }
        }
    }
}

