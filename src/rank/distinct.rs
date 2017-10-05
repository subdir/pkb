use sequential::{Sequential, LowBound};
use rank::intersect::Intersect;


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Distinct<P, S> {
    primary: P,
    secondary: S
}


impl<P, S> Distinct<P, S>
where
    S: Intersect<P>
{
    pub fn new(primary: P, secondary: S) -> Self {
        assert!(!secondary.intersects_with(&primary));
        Self { primary: primary, secondary: secondary }
    }
}


impl<P, S> Distinct<P, S>
where
    P: Clone,
    S: Clone
{
    pub fn primary(&self) -> P { self.primary.clone() }
    pub fn secondary(&self) -> S { self.secondary.clone() }
}


impl<P, S> Sequential for Distinct<P, S>
where
    P: Sequential + Clone,
    S: Sequential + LowBound + Intersect<P> + Clone
{
    fn consequent(&self) -> Option<Self> {
        match self.secondary.consequent().and_then(|c| c.skip_intersecting(&self.primary)) {
            Some(next_secondary) => Some(Self::new(self.primary.clone(), next_secondary)),
            None => match self.primary.consequent() {
                None => None,
                Some(next_primary) => {
                    match S::lowest().skip_intersecting(&next_primary) {
                        Some(next_secondary) => Some(Self::new(next_primary, next_secondary)),
                        None => None
                    }
                }
            }
        }
    }
}


impl<T, H, P> Intersect<T> for Distinct<H, P>
where
    H: Intersect<T>,
    P: Intersect<T>
{
    fn intersects_with(&self, other: &T) -> bool {
        self.primary.intersects_with(other)
        || self.secondary.intersects_with(other)
    }
}

