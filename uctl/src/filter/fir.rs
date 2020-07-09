/*!

## FIR filter

This module implements **Finite Impulse Response** (FIR) filter.

The parameters of filter can be found using different analytical methods and it's non-trivial.

See also [Finite impulse response](https://en.wikipedia.org/wiki/Finite_impulse_response).

*/

use crate::{DelayLine, Transducer};
use core::{
    marker::PhantomData,
    ops::{Add, Mul},
};
use generic_array::{ArrayLength, GenericArray};
use typenum::{Add1, NonZero, Prod, Unsigned, B1};
use ufix::Cast;

/// FIR filter parameters
///
/// - `B` - filter weights type
/// - `N` - filter order
pub type Param<B, N> = GenericArray<B, Add1<N>>;

/// FIR filter state
///
/// - `L` - delay line type
///
/// The input type of filter depended from delayline.
pub type State<L> = L;

/// FIR filter
///
/// - `O` - output values type
/// - `B` - filter weights type
/// - `L` - delay line type
///
pub struct Filter<O, B, L>(PhantomData<(O, B, L)>);

impl<O, B, L> Transducer for Filter<O, B, L>
where
    B: Copy + Mul<L::Value>,
    O: Cast<Prod<B, L::Value>> + Add<O, Output = O>,
    L: DelayLine,
    for<'a> &'a L: IntoIterator<Item = L::Value>,
    L::Length: Add<B1>,
    Add1<L::Length>: ArrayLength<B> + NonZero + Unsigned,
{
    type Input = L::Value;
    type Output = O;
    type Param = Param<B, L::Length>;
    type State = State<L>;

    fn apply(param: &Self::Param, state: &mut Self::State, value: Self::Input) -> Self::Output {
        let result = param
            .iter()
            .skip(1)
            .zip(state.iter())
            .fold(O::cast(param[0] * value), |accum, (b, x)| {
                accum + O::cast(*b * x)
            });

        state.push(value);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pfdl::Store as DL;
    use typenum::{P16, P8, U3};
    use ufix::si;

    #[test]
    fn fir_i8_n3() {
        let param = Param::<i8, U3>::from([9, 1, 7, 4]);
        let mut state = DL::<i8, U3>::from(0);

        type Filter1 = Filter<i8, i8, DL<i8, U3>>;

        assert_eq!(Filter1::apply(&param, &mut state, 0), 0);
        assert_eq!(Filter1::apply(&param, &mut state, 1), 9);
        assert_eq!(Filter1::apply(&param, &mut state, 0), 1);
        assert_eq!(Filter1::apply(&param, &mut state, 0), 7);
        assert_eq!(Filter1::apply(&param, &mut state, 0), 4);
        assert_eq!(Filter1::apply(&param, &mut state, 0), 0);
        assert_eq!(Filter1::apply(&param, &mut state, -1), -9);
        assert_eq!(Filter1::apply(&param, &mut state, 1), 8);
        assert_eq!(Filter1::apply(&param, &mut state, 3), 21);
        assert_eq!(Filter1::apply(&param, &mut state, -7), -57);
        assert_eq!(Filter1::apply(&param, &mut state, -7), -45);
        assert_eq!(Filter1::apply(&param, &mut state, 10), 46);
    }

    #[test]
    fn fir_fix_base10_n3() {
        type I = si::Micro<P8>;
        type O = si::Nano<P16>;
        type P = si::Milli<P8>;

        let param =
            Param::<P, U3>::from([P::new(0_456), P::new(-0_137), P::new(0_702), P::new(-1_421)]);
        let mut state = DL::from(I::new(0));

        type Filter1 = Filter<O, P, DL<I, U3>>;

        assert_eq!(
            Filter1::apply(&param, &mut state, I::new(0_000)),
            O::new(0_000)
        );
        assert_eq!(
            Filter1::apply(&param, &mut state, I::new(1_000)),
            O::new(456_000)
        );
        assert_eq!(
            Filter1::apply(&param, &mut state, I::new(0_000)),
            O::new(-137_000)
        );
        assert_eq!(
            Filter1::apply(&param, &mut state, I::new(0_000)),
            O::new(702_000)
        );
        assert_eq!(
            Filter1::apply(&param, &mut state, I::new(0_000)),
            O::new(-1_421_000)
        );
        assert_eq!(
            Filter1::apply(&param, &mut state, I::new(0_000)),
            O::new(0_000)
        );

        assert_eq!(
            Filter1::apply(&param, &mut state, I::new(0_123)),
            O::new(56_088)
        );
        assert_eq!(
            Filter1::apply(&param, &mut state, I::new(11_234)),
            O::new(5_105_853)
        );
        assert_eq!(
            Filter1::apply(&param, &mut state, I::new(5_001)),
            O::new(827_744)
        );
        assert_eq!(
            Filter1::apply(&param, &mut state, I::new(-3_120)),
            O::new(5_603_628)
        );
        assert_eq!(
            Filter1::apply(&param, &mut state, I::new(-8_998)),
            O::new(-16_128_460)
        );
    }
}
