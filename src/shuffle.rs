use rand::Rng;


pub struct LazyShuffledVec<T> {
    vec: Vec<T>,
    taken_count: usize,
}


impl<T: Clone> LazyShuffledVec<T> {
    pub fn new(vec: Vec<T>) -> Self {
        Self { vec: vec, taken_count: 0 }
    }

    pub fn shuffled_iter<R: Rng>(&mut self, rng: R) -> LazyShuffledIter<T, R> {
        assert_eq!(self.taken_count, 0);
        LazyShuffledIter{ vec: self, rng: rng }
    }

    fn next_rand<R: Rng>(&mut self, rng: &mut R) -> Option<T> {
        debug_assert!(self.taken_count <= self.vec.len());
        if self.taken_count == self.vec.len() {
            None
        } else {
            let taken = rng.gen_range(self.taken_count, self.vec.len());
            self.vec.swap(self.taken_count, taken);
            self.taken_count += 1;
            Some(self.vec[self.taken_count - 1].clone())
        }
    }

    fn reset(&mut self) {
        self.taken_count = 0;
    }
}


pub struct LazyShuffledIter<'a, T: Clone + 'a, R: Rng + 'a> {
    vec: &'a mut LazyShuffledVec<T>,
    rng: R
}


impl<'a, T: Clone + 'a, R: Rng> Iterator for LazyShuffledIter<'a, T, R> {
    type Item=T;

    fn next(&mut self) -> Option<T> {
        self.vec.next_rand(&mut self.rng)
    }
}


impl<'a, T: Clone + 'a, R: Rng> Drop for LazyShuffledIter<'a, T, R> {
    fn drop(&mut self) {
        self.vec.reset();
    }
}


#[cfg(test)]
mod tests {
    use rand;
    use rand::SeedableRng;
    use super::*;

    #[test]
    fn test() {
        let rng = rand::XorShiftRng::from_seed([1, 2, 3, 4]);
        let mut vec = LazyShuffledVec::new(vec!(0, 1, 2, 3, 4, 5, 6));
        assert_eq!(
            vec.shuffled_iter(rng.clone()).take(5).collect::<Vec<u8>>(),
            vec!(6, 3, 4, 5, 1)
        );
        assert_eq!(
            vec.shuffled_iter(rng.clone()).collect::<Vec<u8>>(),
            vec!(0, 5, 1, 2, 3, 4, 6)
        );
    }
}

