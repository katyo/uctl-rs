use super::DelayLine;

/// Delay line to use as internal storage for filters
#[derive(Debug, Default)]
pub struct FillableDelayLine<T, N> where N: ArrayLength<T> + NonZero {
    /// Statically sized storage for all available values
    data: GenericArray<T, N>,
    /// The number of actually stored values
    fill: usize,
    /// The position after of the last pushed value
    tail: usize,
}

impl<T, N> FillableDelayLine<T, N>
where N: ArrayLength<T> + NonZero,
{
    /// Returns current number of values
    pub fn len(&self) -> usize {
        self.fill
    }

    /// Returns maximum number of values
    pub fn max_len() -> usize {
        N::to_usize()
    }

    /// Delay line is empty
    ///
    /// Returns true when `.len` == 0
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Delay line if full
    ///
    /// Returns true when `.len` == `.max_len`
    pub fn is_full(&self) -> bool {
        self.len() == Self::max_len()
    }

    /// Push new value to delay line
    pub fn push(&mut self, value: T) {
        self.data[self.tail] = value;
        if self.fill < Self::max_len() {
            self.fill += 1;
        }
        self.tail += 1;
        Self::wrap(&mut self.tail);
    }

    /*
    /// Pop value from delay line
    pub fn shift(&mut self) {
        if self.fill > 0 {
            self.fill -= 1;
        }
    }
     */

    /// Get iterator over stored values
    pub fn iter<'a>(&'a self) -> DelayLineIter<'a, T, N> {
        self.into_iter()
    }

    /// Get position of first element of line
    fn head(&self) -> usize {
        if self.fill > 0 {
            if self.tail < self.fill {
                self.tail + Self::max_len() - self.fill
            } else {
                self.tail - self.fill
            }
        } else {
            usize::MAX
        }
    }

    fn wrap(item: &mut usize) {
        while *item >= Self::max_len() {
            *item -= Self::max_len();
        }
    }

    fn last(&self) -> usize {
        if self.fill > 0 {
            if self.tail > 0 {
                self.tail - 1
            } else {
                Self::max_len() - 1
            }
        } else {
            usize::MAX
        }
    }
}

impl<'a, T, N> IntoIterator for &'a DelayLine<T, N>
where N: ArrayLength<T> + NonZero,
{
    type Item = &'a T;
    type IntoIter = DelayLineIter<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        DelayLineIter {
            line: self,
            head: self.head(),
            item: self.last(),
        }
    }
}

/// Iterator over stored values
pub struct DelayLineIter<'a, T, N>
where N: ArrayLength<T> + NonZero,
{
    /// Delay line
    line: &'a DelayLine<T, N>,
    /// First element position
    head: usize,
    /// Current position
    item: usize,
}

impl<'a, T, N> Iterator for DelayLineIter<'a, T, N>
where N: ArrayLength<T> + NonZero,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.item != self.head {
            let item = self.item;

            if self.item > 0 {
                self.item -= 1;
            } else {
                self.item = DelayLine::<T, N>::max_len() - 1;
            }

            Some(&self.line.data[item])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{U1, U3};
    use super::*;

    #[test]
    fn max_len() {
        assert_eq!(DelayLine::<i8, U1>::max_len(), 1);
        assert_eq!(DelayLine::<i8, U3>::max_len(), 3);
    }

    #[test]
    fn len() {
        let mut dl = DelayLine::<i8, U3>::default();

        assert_eq!(dl.len(), 0);
        dl.push(1);
        assert_eq!(dl.len(), 1);
        dl.push(2);
        assert_eq!(dl.len(), 2);
        dl.push(3);
        assert_eq!(dl.len(), 3);
        dl.push(4);
        assert_eq!(dl.len(), 3);
    }

    #[test]
    fn iter() {
        let mut dl = DelayLine::<i8, U3>::default();

        assert_eq!(dl.iter().size_hint(), 0);
        dl.push(1);
        assert_eq!(dl.len(), 1);
        dl.push(2);
        assert_eq!(dl.len(), 2);
        dl.push(3);
        assert_eq!(dl.len(), 3);
        dl.push(4);
        assert_eq!(dl.len(), 3);
    }
}
