use core::fmt;
use core::iter::FusedIterator;

/// TakeUntilExt is an extension trait for iterators.
/// It adds the `take_until_inclusive` method.
pub trait TakeUntilInclusiveExt<P>
where
    Self: Sized,
{
    fn take_until_inclusive(self, predicate: P) -> TakeUntilInclusive<Self, P>;
}

impl<I, P> TakeUntilInclusiveExt<P> for I
where
    I: Sized + Iterator,
    P: FnMut(&I::Item) -> bool,
{
    fn take_until_inclusive(self, predicate: P) -> TakeUntilInclusive<Self, P> {
        TakeUntilInclusive {
            iter: self,
            flag: false,
            predicate,
        }
    }
}
/// TakeUntil is similar to the TakeWhile iterator,
/// but takes items until the predicate is true,
/// including the item that made the predicate true.
pub struct TakeUntilInclusive<I, P> {
    iter: I,
    flag: bool,
    predicate: P,
}

impl<I: fmt::Debug, P> fmt::Debug for TakeUntilInclusive<I, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TakeUntilInclusive")
            .field("iter", &self.iter)
            .field("flag", &self.flag)
            .finish()
    }
}

impl<I, P> Iterator for TakeUntilInclusive<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if self.flag {
            None
        } else {
            self.iter.next().and_then(|x| {
                if (self.predicate)(&x) {
                    self.flag = true;
                }
                Some(x)
            })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.flag {
            (0, Some(0))
        } else {
            let (_, upper) = self.iter.size_hint();
            (0, upper) // can't know a lower bound, due to the predicate
        }
    }
}

impl<I, P> FusedIterator for TakeUntilInclusive<I, P>
where
    I: FusedIterator,
    P: FnMut(&I::Item) -> bool,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_size_hint_zero() {
        let v: Vec<u8> = vec![0, 1, 2];
        let mut iter = v.iter().take_until_inclusive(|_| true);
        assert_eq!((0, Some(3)), iter.size_hint());
        iter.next();
        assert_eq!((0, Some(0)), iter.size_hint());
    }

    // #[test]
    // fn test_size_hint() {
    //     let iter = (0..10);

    // }
}
