use rand::Rng;

use Card;
use shuffle::LazyShuffledVec;
use shuffle::LazyShuffledIter;


pub struct Deck {
    vec: LazyShuffledVec<Card>
}


impl Deck {
    pub fn new() -> Self {
        Self { vec: LazyShuffledVec::new(Card::gen_deck().collect()) }
    }

    pub fn shuffled_iter<R: Rng>(&mut self, rng: R) -> LazyShuffledIter<Card, R> {
        self.vec.shuffled_iter(rng)
    }
}

