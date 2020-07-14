pub mod pfdl;

use typenum::{NonZero, Unsigned};

/// Delay line to use as internal storage for filters
pub trait DelayLine
where
    for<'a> &'a Self: IntoIterator<Item = <Self as DelayLine>::Value>,
{
    /// The type of element
    type Value: Copy;
    /// The length of line
    type Length: Unsigned + NonZero;

    /// Push new value to line
    fn push(&mut self, value: Self::Value);

    /// Current number of values in line
    fn len(&self) -> usize;

    /// Maximum number of values in line
    fn max_len() -> usize {
        Self::Length::to_usize()
    }

    /// Delay line is empty
    ///
    /// Returns true when `.len` == 0
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Delay line if full
    ///
    /// Returns true when `.len` == `.max_len`
    fn is_full(&self) -> bool {
        self.len() == Self::max_len()
    }

    /// Get iterator over a stored values
    fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}
