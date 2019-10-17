/*!

## Pre-filled delay line

This module implements delay line which pre-initialized with some value.

*/

use crate::{NonZero, GenericArray, ArrayLength, IntoIterator, FromIterator, usize, repeat};
use super::{DelayLine};

/// Simple pre-filled delay line
#[derive(Debug, Default)]
pub struct Store<T, N>
where T: Copy,
      N: ArrayLength<T> + NonZero
{
    /// Statically sized storage for all available values
    data: GenericArray<T, N>,
    /// The position after of the last pushed value
    tail: usize,
}

impl<T, N> From<T> for Store<T, N>
where T: Copy,
      N: ArrayLength<T> + NonZero,
{
    fn from(value: T) -> Self {
        Self {
            data: FromIterator::from_iter(repeat(value).take(Self::max_len())),
            tail: 0,
        }
    }
}

impl<T, N> DelayLine for Store<T, N>
where T: Copy,
      N: ArrayLength<T> + NonZero,
{
    type Value = T;
    type Length = N;

    fn push(&mut self, value: Self::Value) {
        self.data[self.tail] = value;
        self.tail += 1;
        if self.tail == Self::max_len() {
            self.tail = 0;
        }
    }

    fn len(&self) -> usize {
        Self::max_len()
    }
}

impl<'a, T, N> IntoIterator for &'a Store<T, N>
where T: Copy,
      N: ArrayLength<T> + NonZero,
{
    type Item = T;
    type IntoIter = Iter<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            line: self,
            item: self.tail,
        }
    }
}

/// Iterator over stored values
pub struct Iter<'a, T, N>
where T: Copy,
      N: ArrayLength<T> + NonZero,
{
    /// Delay line
    line: &'a Store<T, N>,
    /// Current position
    item: usize,
}

impl<'a, T, N> Iterator for Iter<'a, T, N>
where T: Copy,
      N: ArrayLength<T> + NonZero,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.item != usize::MAX {
            if self.item > 0 {
                self.item -= 1;
            } else {
                self.item = Store::<T, N>::max_len() - 1;
            }

            let item = self.item;

            if self.item == self.line.tail {
                self.item = usize::MAX;
            }

            Some(self.line.data[item])
        } else {
            None
        }
    }
}

impl<'a, T, N> ExactSizeIterator for Iter<'a, T, N>
where T: Copy,
      N: ArrayLength<T> + NonZero,
{
    fn len(&self) -> usize {
        if self.item != usize::MAX {
            if self.item <= self.line.tail {
                Store::<T, N>::max_len() - self.line.tail + self.item
            } else {
                self.item - self.line.tail
            }
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{U1, U3};
    use super::*;

    #[test]
    fn max_len() {
        assert_eq!(Store::<i8, U1>::max_len(), 1);
        assert_eq!(Store::<i8, U3>::max_len(), 3);
    }

    #[test]
    fn len() {
        let mut dl = Store::<i8, U3>::default();

        assert_eq!(dl.len(), 3);
        dl.push(1);
        assert_eq!(dl.len(), 3);
        dl.push(2);
        assert_eq!(dl.len(), 3);
        dl.push(3);
        assert_eq!(dl.len(), 3);
        dl.push(4);
        assert_eq!(dl.len(), 3);
    }

    #[test]
    fn iter_count() {
        let mut dl = Store::<i8, U3>::default();

        assert_eq!(dl.iter().count(), 3);
        dl.push(1);
        assert_eq!(dl.iter().count(), 3);
        dl.push(2);
        assert_eq!(dl.iter().count(), 3);
        dl.push(3);
        assert_eq!(dl.iter().count(), 3);
        dl.push(4);
        assert_eq!(dl.iter().count(), 3);
        dl.push(5);
        assert_eq!(dl.iter().count(), 3);
    }

    #[test]
    fn iter_len() {
        let dl = Store::<i8, U3>::default();

        {
            let mut it = dl.iter();
            assert_eq!(it.len(), 3);
            assert_eq!(it.next(), Some(0));
            assert_eq!(it.len(), 2);
            assert_eq!(it.next(), Some(0));
            assert_eq!(it.len(), 1);
            assert_eq!(it.next(), Some(0));
            assert_eq!(it.len(), 0);
            assert_eq!(it.next(), None);
            assert_eq!(it.len(), 0);
            assert_eq!(it.next(), None);
        }
    }

    #[test]
    fn iter() {
        let mut dl = Store::<i8, U3>::default();

        {
            let mut it = dl.iter();
            assert_eq!(it.next(), Some(0));
            assert_eq!(it.next(), Some(0));
            assert_eq!(it.next(), Some(0));
            assert_eq!(it.next(), None);
            assert_eq!(it.next(), None);
        }
        dl.push(1);
        {
            let mut it = dl.iter();
            assert_eq!(it.next(), Some(1));
            assert_eq!(it.next(), Some(0));
            assert_eq!(it.next(), Some(0));
            assert_eq!(it.next(), None);
            assert_eq!(it.next(), None);
        }
        dl.push(2);
        {
            let mut it = dl.iter();
            assert_eq!(it.next(), Some(2));
            assert_eq!(it.next(), Some(1));
            assert_eq!(it.next(), Some(0));
            assert_eq!(it.next(), None);
            assert_eq!(it.next(), None);
        }
        dl.push(3);
        {
            let mut it = dl.iter();
            assert_eq!(it.next(), Some(3));
            assert_eq!(it.next(), Some(2));
            assert_eq!(it.next(), Some(1));
            assert_eq!(it.next(), None);
            assert_eq!(it.next(), None);
        }
        dl.push(4);
        {
            let mut it = dl.iter();
            assert_eq!(it.next(), Some(4));
            assert_eq!(it.next(), Some(3));
            assert_eq!(it.next(), Some(2));
            assert_eq!(it.next(), None);
            assert_eq!(it.next(), None);
        }
        dl.push(5);
        {
            let mut it = dl.iter();
            assert_eq!(it.next(), Some(5));
            assert_eq!(it.next(), Some(4));
            assert_eq!(it.next(), Some(3));
            assert_eq!(it.next(), None);
            assert_eq!(it.next(), None);
        }
        dl.push(6);
        {
            let mut it = dl.iter();
            assert_eq!(it.next(), Some(6));
            assert_eq!(it.next(), Some(5));
            assert_eq!(it.next(), Some(4));
            assert_eq!(it.next(), None);
            assert_eq!(it.next(), None);
        }
        dl.push(7);
        {
            let mut it = dl.iter();
            assert_eq!(it.next(), Some(7));
            assert_eq!(it.next(), Some(6));
            assert_eq!(it.next(), Some(5));
            assert_eq!(it.next(), None);
            assert_eq!(it.next(), None);
        }
        dl.push(8);
        {
            let mut it = dl.iter();
            assert_eq!(it.next(), Some(8));
            assert_eq!(it.next(), Some(7));
            assert_eq!(it.next(), Some(6));
            assert_eq!(it.next(), None);
            assert_eq!(it.next(), None);
        }
        dl.push(9);
        {
            let mut it = dl.iter();
            assert_eq!(it.next(), Some(9));
            assert_eq!(it.next(), Some(8));
            assert_eq!(it.next(), Some(7));
            assert_eq!(it.next(), None);
            assert_eq!(it.next(), None);
        }
        dl.push(10);
        {
            let mut it = dl.iter();
            assert_eq!(it.next(), Some(10));
            assert_eq!(it.next(), Some(9));
            assert_eq!(it.next(), Some(8));
            assert_eq!(it.next(), None);
            assert_eq!(it.next(), None);
        }
    }

    #[test]
    fn from_value() {
        let dl = Store::<i8, U3>::from(11);

        for item in &dl {
            assert_eq!(item, 11);
        }
    }
}
