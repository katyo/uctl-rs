use crate::Transducer;
use core::{
    marker::PhantomData,
    //ops::{RangeFrom, RangeToInclusive, RangeInclusive, RangeFull},
    ops::{Bound::*, RangeBounds},
};

pub struct Clamper<R, T> {
    val: PhantomData<(R, T)>,
}

impl<R, T> Transducer for Clamper<R, T>
where
    R: RangeBounds<T>,
    T: Copy + PartialOrd,
{
    type Input = T;
    type Output = T;
    type Param = R;
    type State = ();

    #[inline]
    fn apply(param: &Self::Param, _state: &mut Self::State, value: Self::Input) -> Self::Output {
        let value = match param.start_bound() {
            Included(from) => {
                if value < *from {
                    *from
                } else {
                    value
                }
            }
            Excluded(from) => {
                if value <= *from {
                    *from
                } else {
                    value
                }
            }
            Unbounded => value,
        };
        match param.end_bound() {
            Included(to) => {
                if value > *to {
                    *to
                } else {
                    value
                }
            }
            Excluded(to) => {
                if value >= *to {
                    *to
                } else {
                    value
                }
            }
            Unbounded => value,
        }
    }
}

/*
impl<T> Transducer for Clamper<RangeFrom<T>, T>
where
    T: Copy + PartialOrd,
{
    type Input = T;
    type Output = T;
    type Param = RangeFrom<T>;
    type State = ();

    #[inline]
    fn apply(param: &Self::Param, _state: &mut Self::State, value: Self::Input) -> Self::Output {
        if value < param.start {
            param.start
        } else {
            value
        }
    }
}

impl<T> Transducer for Clamper<RangeToInclusive<T>, T>
where
    T: Copy + PartialOrd,
{
    type Input = T;
    type Output = T;
    type Param = RangeToInclusive<T>;
    type State = ();

    #[inline]
    fn apply(param: &Self::Param, _state: &mut Self::State, value: Self::Input) -> Self::Output {
        if value > param.end {
            param.end
        } else {
            value
        }
    }
}

impl<T> Transducer for Clamper<RangeInclusive<T>, T>
where
    T: Copy + PartialOrd,
{
    type Input = T;
    type Output = T;
    type Param = RangeInclusive<T>;
    type State = ();

    #[inline]
    fn apply(param: &Self::Param, _state: &mut Self::State, value: Self::Input) -> Self::Output {
        let value = if value < *param.start() {
            *param.start()
        } else {
            value
        };
        if value > *param.end() {
            *param.end()
        } else {
            value
        }
    }
}

impl<T> Transducer for Clamper<RangeFull, T> {
    type Input = T;
    type Output = T;
    type Param = RangeFull;
    type State = ();

    #[inline]
    fn apply(param: &Self::Param, _state: &mut Self::State, value: Self::Input) -> Self::Output {
        value
    }
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn clamp_from_float() {
        let range = 3.0..;

        assert_eq!(Clamper::apply(&range, &mut (), 2.9), 3.0);
        assert_eq!(Clamper::apply(&range, &mut (), 3.0), 3.0);
        assert_eq!(Clamper::apply(&range, &mut (), 3.1), 3.1);
    }

    #[test]
    fn clamp_to_float() {
        let range = ..=3.0;

        assert_eq!(Clamper::apply(&range, &mut (), 2.9), 2.9);
        assert_eq!(Clamper::apply(&range, &mut (), 3.0), 3.0);
        assert_eq!(Clamper::apply(&range, &mut (), 3.1), 3.0);
    }

    #[test]
    fn clamp_in_float() {
        let range = 2.0..=3.0;

        assert_eq!(Clamper::apply(&range, &mut (), 1.9), 2.0);
        assert_eq!(Clamper::apply(&range, &mut (), 2.0), 2.0);
        assert_eq!(Clamper::apply(&range, &mut (), 2.1), 2.1);
        assert_eq!(Clamper::apply(&range, &mut (), 2.9), 2.9);
        assert_eq!(Clamper::apply(&range, &mut (), 3.0), 3.0);
        assert_eq!(Clamper::apply(&range, &mut (), 3.1), 3.0);
    }
}
