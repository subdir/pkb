use std::iter::Peekable;


pub trait StrictAscIter where Self: Iterator, Self::Item: Ord {}
pub trait StrictDescIter where Self: Iterator, Self::Item: Ord {}


pub trait SortedIterator {
    fn ensure_strict_asc(self) -> EnsureStrictAsc<Self>
    where
        Self: Iterator + Sized,
        Self::Item: Ord
    {
        EnsureStrictAsc::new(self)
    }

    fn ensure_strict_desc(self) -> EnsureStrictDesc<Self>
    where
        Self: Iterator + Sized,
        Self::Item: Ord
    {
        EnsureStrictDesc::new(self)
    }
}


impl<T> SortedIterator for T where T: Iterator {}


pub struct EnsureStrictAsc<I> where I: Iterator, I::Item: Ord {
    iter: Peekable<I>
}

impl<I> EnsureStrictAsc<I> where I: Iterator, I::Item: Ord {
    fn new(iter: I) -> Self { Self { iter: iter.peekable() } }
}

impl<I> StrictAscIter for EnsureStrictAsc<I> where I: Iterator, I::Item: Ord {}

impl<I> Iterator for EnsureStrictAsc<I> where I: Iterator, I::Item: Ord {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        ensured_order_next(&mut self.iter, |a, b| a < b)
    }
}


pub struct EnsureStrictDesc<I> where I: Iterator, I::Item: Ord {
    iter: Peekable<I>
}

impl<I> EnsureStrictDesc<I> where I: Iterator, I::Item: Ord {
    fn new(iter: I) -> Self { Self { iter: iter.peekable() } }
}

impl<I> StrictDescIter for EnsureStrictDesc<I> where I: Iterator, I::Item: Ord {}

impl<I> Iterator for EnsureStrictDesc<I> where I: Iterator, I::Item: Ord {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        ensured_order_next(&mut self.iter, |a, b| a > b)
    }
}


fn ensured_order_next<I, F>(iter: &mut Peekable<I>, assertion: F) -> Option<I::Item>
where
    I: Iterator,
    F: Fn(&I::Item, &I::Item) -> bool,
{
    match iter.next() {
        None => None,
        Some(next) => match iter.peek() {
            None => Some(next),
            Some(peek) => {
                assert!(assertion(&next, &peek));
                Some(next)
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
            vec!(1u8,2,3).into_iter().ensure_strict_asc().collect::<Vec<u8>>(),
            vec!(1u8,2,3)
        );
    }

    #[test]
    #[should_panic(expected = "assertion failed: assertion(&next, &peek)")]
    fn test_asc_fail() {
        vec!(1u8,2,2).into_iter().ensure_strict_asc().collect::<Vec<u8>>();
    }

    #[test]
    fn test_desc_ok() {
        assert_eq!(
            vec!(3u8,2,1).into_iter().ensure_strict_desc().collect::<Vec<u8>>(),
            vec!(3u8,2,1)
        );
    }

    #[test]
    #[should_panic(expected = "assertion failed: assertion(&next, &peek)")]
    fn test_desc_fail() {
        vec!(3u8,2,2).into_iter().ensure_strict_desc().collect::<Vec<u8>>();
    }
}

