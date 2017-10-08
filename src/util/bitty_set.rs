use countable::Countable;


trait Bits: Sized {
    fn zero() -> Self;
    fn size() -> usize;
    fn get(&mut self, bit: usize) -> bool;
    fn set(&mut self, bit: usize);
    fn set_bits_desc_iter(self) -> SetBitsDescIter<Self> {
        SetBitsDescIter { bits: self, index: Self::size() - 1 }
    }
}


struct SetBitsDescIter<B: Bits> {
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


#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Hash)]
struct TinySet<T: Countable, B: Bits + Clone> {
    bits: B,
    type_holder: Option<T>
}


impl<T: Countable, B: Bits + Clone> TinySet<T, B> {
    fn new() -> Self { Self { bits: B::zero(), type_holder: None } }

    fn add(&mut self, val: &T) {
        self.bits.set(val.to_serial())
    }

    fn contains(&mut self, val: &T) -> bool {
        self.bits.get(val.to_serial())
    }

    fn desc_iter(&self) -> impl Iterator<Item=T> {
        self.bits.clone().set_bits_desc_iter().map(|b| T::from_serial(b))
    }
}


/*
struct TinyBitSetIter<T: Bits> {
    bits: T
}


#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Hash)]
struct TinyBitSet {
    bits: u64
}


impl<T: Countable> TinySet {
    fn new() -> Self { Self { bits: 0 } }

    fn add(&self, val: &T) {
        let serial = val.to_serial();
        assert!(serial < u64::bits());
        self.bits &= 1 << val.to_serial();
    }

    fn contains(&self, val: &T) -> bool {
        (self.bits & (1 << val.to_serial())) != 0
    }

//    fn iter(&self) -> TinyBitSetIter<T> {
  //  }
}

*/


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bits() {
        let mut bits = 0u64;
        bits.set(2);
        assert_eq!(bits, 4);
        bits.set(4);
        assert_eq!(vec!(4, 2), bits.set_bits_desc_iter().collect::<Vec<usize>>());
    }
}

