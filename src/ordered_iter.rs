use std::iter::Peekable;


pub trait OrderedIterator
where
    Self: Iterator,
    Self::Item: Ord,
    Self::Order: Order
{
    type Order;
}


pub trait OrderEnsurableIterator {
    fn ensure_order<O>(self, order: O) -> EnsureOrder<O, Self>
    where
        Self: Iterator + Sized,
        Self::Item: Ord,
        O: Order
    {
        EnsureOrder::new(order, self)
    }
}


impl<T> OrderEnsurableIterator for T where T: Iterator {}


pub trait Order {
    fn holds<T: Ord>(&self, a: &T, b: &T) -> bool;
}


pub struct Ascending;
impl Order for Ascending {
    fn holds<T: Ord>(&self, a: &T, b: &T) -> bool { a <= b }
}


pub struct Descending;
impl Order for Descending {
    fn holds<T: Ord>(&self, a: &T, b: &T) -> bool { a >= b }
}


pub struct StrictAscending;
impl Order for StrictAscending {
    fn holds<T: Ord>(&self, a: &T, b: &T) -> bool { a < b }
}


pub struct StrictDescending;
impl Order for StrictDescending {
    fn holds<T: Ord>(&self, a: &T, b: &T) -> bool { a > b }
}


pub struct EnsureOrder<O, I>
where
    I: Iterator,
    I::Item: Ord,
    O: Order
{
    iter: Peekable<I>,
    order: O
}


impl<O, I> EnsureOrder<O, I>
where
    I: Iterator,
    I::Item: Ord,
    O: Order
{
    fn new(order: O, iter: I) -> Self { Self { iter: iter.peekable(), order: order } }
}


impl<O, I> OrderedIterator for EnsureOrder<O, I>
where
    I: Iterator,
    I::Item: Ord,
    O: Order
{
    type Order = O;
}


impl<O, I> Iterator for EnsureOrder<O, I>
where
    I: Iterator,
    I::Item: Ord,
    O: Order
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(next) => match self.iter.peek() {
                None => Some(next),
                Some(peek) => {
                    assert!(self.order.holds(&next, &peek));
                    Some(next)
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asc_ok() {
        assert_eq!(
            vec!(1u8,2,2).into_iter().ensure_order(Ascending).collect::<Vec<u8>>(),
            vec!(1u8,2,2)
        );
    }

    #[test]
    #[should_panic(expected = "assertion failed: self.order.holds(&next, &peek)")]
    fn test_asc_fail() {
        vec!(1u8,2,1).into_iter().ensure_order(Ascending).collect::<Vec<u8>>();
    }

    #[test]
    fn test_strict_asc_ok() {
        assert_eq!(
            vec!(1u8,2,3).into_iter().ensure_order(StrictAscending).collect::<Vec<u8>>(),
            vec!(1u8,2,3)
        );
    }

    #[test]
    #[should_panic(expected = "assertion failed: self.order.holds(&next, &peek)")]
    fn test_strict_asc_fail() {
        vec!(1u8,2,2).into_iter().ensure_order(StrictAscending).collect::<Vec<u8>>();
    }

    #[test]
    fn test_desc_ok() {
        assert_eq!(
            vec!(3u8,3,1).into_iter().ensure_order(Descending).collect::<Vec<u8>>(),
            vec!(3u8,3,1)
        );
    }

    #[test]
    #[should_panic(expected = "assertion failed: self.order.holds(&next, &peek)")]
    fn test_desc_fail() {
        vec!(3u8,2,3).into_iter().ensure_order(Descending).collect::<Vec<u8>>();
    }

    #[test]
    fn test_strict_desc_ok() {
        assert_eq!(
            vec!(3u8,2,1).into_iter().ensure_order(StrictDescending).collect::<Vec<u8>>(),
            vec!(3u8,2,1)
        );
    }

    #[test]
    #[should_panic(expected = "assertion failed: self.order.holds(&next, &peek)")]
    fn test_strict_desc_fail() {
        vec!(3u8,2,2).into_iter().ensure_order(StrictDescending).collect::<Vec<u8>>();
    }
}

