use std::iter::Peekable;


pub trait OrderedIterator
where
    Self: Iterator,
    Self::Item: Ord,
    Self::Order: Order
{
    type Order;
}


/*
Doesn't work. Possibly related bugs:

    https://github.com/rust-lang/rust/issues/40761
    https://github.com/rust-lang/rust/issues/39959

impl<T> OrderedIterator for T
where
    T: OrderedIterator<Order=StrictAscending, Item=Self::Item>
{
    type Order = Ascending;
}


impl<T> OrderedIterator for T
where
    T: OrderedIterator<Order=StrictDescending, Item=Self::Item>
{
    type Order = Descending;
}
*/


pub trait OrderEnsurableIterator {
    fn assert_order<O>(self, order: O) -> AssertOrder<O, Self>
    where
        Self: Iterator + Sized,
        Self::Item: Ord,
        O: Order
    {
        AssertOrder::new(order, self)
    }
    fn debug_assert_order<O>(self, order: O) -> DebugAssertOrder<O, Self>
    where
        Self: Iterator + Sized,
        Self::Item: Ord,
        O: Order
    {
        DebugAssertOrder::new(order, self)
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


pub struct AssertOrder<O, I>
where
    I: Iterator,
    I::Item: Ord,
    O: Order
{
    iter: Peekable<I>,
    order: O
}


impl<O, I> AssertOrder<O, I>
where
    I: Iterator,
    I::Item: Ord,
    O: Order
{
    fn new(order: O, iter: I) -> Self { Self { iter: iter.peekable(), order: order } }
}


impl<O, I> OrderedIterator for AssertOrder<O, I>
where
    I: Iterator,
    I::Item: Ord,
    O: Order
{
    type Order = O;
}


impl<O, I> Iterator for AssertOrder<O, I>
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


pub struct DebugAssertOrder<O, I>
where
    I: Iterator,
    I::Item: Ord,
    O: Order
{
    iter: Peekable<I>,
    order: O
}


impl<O, I> DebugAssertOrder<O, I>
where
    I: Iterator,
    I::Item: Ord,
    O: Order
{
    fn new(order: O, iter: I) -> Self { Self { iter: iter.peekable(), order: order } }
}


impl<O, I> OrderedIterator for DebugAssertOrder<O, I>
where
    I: Iterator,
    I::Item: Ord,
    O: Order
{
    type Order = O;
}


impl<O, I> Iterator for DebugAssertOrder<O, I>
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
                    debug_assert!(self.order.holds(&next, &peek));
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
            vec!(1u8,2,2).into_iter().assert_order(Ascending).collect::<Vec<u8>>(),
            vec!(1u8,2,2)
        );
    }

    #[test]
    #[should_panic(expected = "assertion failed: self.order.holds(&next, &peek)")]
    fn test_asc_fail() {
        vec!(1u8,2,1).into_iter().assert_order(Ascending).collect::<Vec<u8>>();
    }

    #[test]
    fn test_strict_asc_ok() {
        assert_eq!(
            vec!(1u8,2,3).into_iter().assert_order(StrictAscending).collect::<Vec<u8>>(),
            vec!(1u8,2,3)
        );
    }

    #[test]
    #[should_panic(expected = "assertion failed: self.order.holds(&next, &peek)")]
    fn test_strict_asc_fail() {
        vec!(1u8,2,2).into_iter().assert_order(StrictAscending).collect::<Vec<u8>>();
    }

    #[test]
    fn test_desc_ok() {
        assert_eq!(
            vec!(3u8,3,1).into_iter().assert_order(Descending).collect::<Vec<u8>>(),
            vec!(3u8,3,1)
        );
    }

    #[test]
    #[should_panic(expected = "assertion failed: self.order.holds(&next, &peek)")]
    fn test_desc_fail() {
        vec!(3u8,2,3).into_iter().assert_order(Descending).collect::<Vec<u8>>();
    }

    #[test]
    fn test_strict_desc_ok() {
        assert_eq!(
            vec!(3u8,2,1).into_iter().debug_assert_order(StrictDescending).collect::<Vec<u8>>(),
            vec!(3u8,2,1)
        );
    }

    #[test]
    #[should_panic(expected = "assertion failed: self.order.holds(&next, &peek)")]
    fn test_strict_desc_fail() {
        vec!(3u8,2,2).into_iter().debug_assert_order(StrictDescending).collect::<Vec<u8>>();
    }
}

