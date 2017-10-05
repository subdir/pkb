use std::fmt::Debug;

use sequential::{Sequential, LowBound};
use rank::intersect::{Intersect, IntersectOrd};


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct DistinctOrdered<H, L> {
    higher: H,
    lower: L
}


impl<H, L> DistinctOrdered<H, L>
where
    H: Debug + IntersectOrd<L>,
    L: Debug
{
    pub fn new(higher: H, lower: L) -> Self {
        assert!(higher.all_greater_than(&lower), "Failed assertion: {:?} > {:?}", higher, lower);
        Self { higher: higher, lower: lower }
    }
}


impl<H: Clone, L: Clone> DistinctOrdered<H, L> {
    pub fn higher(&self) -> H { self.higher.clone() }
    pub fn lower(&self) -> L { self.lower.clone() }
}


impl<H, L> Sequential for DistinctOrdered<H, L>
where
    H: Debug + Sequential + IntersectOrd<L> + Clone,
    L: Debug + Sequential + LowBound
{
    fn consequent(&self) -> Option<Self> {
        let next_lower = self.lower.consequent().unwrap();

        if self.higher.all_greater_than(&next_lower) {
            Some(Self::new(self.higher.clone(), next_lower))
        } else {
            let next_lower = L::lowest();
            let next_higher = self.higher
                .consequent()
                .and_then(|a| a.skip_to_greater_than(&next_lower));

            match next_higher {
                None => None,
                Some(next_higher) => Some(Self::new(next_higher, next_lower))
            }
        }
    }
}


impl<T, H, L> Intersect<T> for DistinctOrdered<H, L>
where
    H: Intersect<T>,
    L: Intersect<T>
{
    fn intersects_with(&self, other: &T) -> bool {
        self.higher.intersects_with(other)
        || self.lower.intersects_with(other)
    }
}


impl<T, H, L> IntersectOrd<T> for DistinctOrdered<H, L>
where
    L: IntersectOrd<T>
{
    fn all_greater_than(&self, other: &T) -> bool {
        self.lower.all_greater_than(other)
    }
}

