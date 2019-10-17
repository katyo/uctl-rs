/*!

## FIR filter

This module implements generic Finite Impulse Response filter.

See wiki article [FIR](https://en.wikipedia.org/wiki/Finite_impulse_response).

*/

use crate::{DelayLine, GenericArray, ArrayLength, NonZero, Unsigned, Mul, Add, Add1, B1, FromOther, PhantomData};
use super::{Filter};

/// FIR filter parameters
///
/// - `B` - filter weights type
/// - `N` - filter order
pub type Param<B, N> = GenericArray<B, Add1<N>>;

/// FIR filter state
///
/// - `O` - output values type
/// - `B` - filter weights type
/// - `L` - delay line type
/// - `Bx`, `Ix` - internal types for multiplication
///
/// The input type of filter depended from delayline.
#[derive(Debug)]
pub struct State<'s, O, B, L, Ix, Bx>
where L: DelayLine,
      for<'a> &'a L: IntoIterator<Item = L::Value>,
      L::Length: Add<B1>,
      Add1<L::Length>: ArrayLength<B> + NonZero + Unsigned,
{
    bi: &'s Param<B, L::Length>,
    xi: L,
    vi: PhantomData<(O, Bx, Ix)>,
}

impl<'s, O, B, L, Ix, Bx> State<'s, O, B, L, Ix, Bx>
where B: Copy,
      L: DelayLine,
      for<'a> &'a L: IntoIterator<Item = L::Value>,
      L::Length: Add<B1>,
      Add1<L::Length>: ArrayLength<B> + NonZero + Unsigned,
{
    pub fn new(bi: &'s Param<B, L::Length>, xi: L) -> Self {
        Self { bi, xi, vi: PhantomData }
    }
}

impl<'s, O, B, L, Ix, Bx> Filter for State<'s, O, B, L, Ix, Bx>
where B: Copy,
      Bx: FromOther<B> + Mul<Ix>,
      Ix: FromOther<L::Value>,
      O: FromOther<<Bx as Mul<Ix>>::Output> + Add<O, Output = O>,
      L: DelayLine,
      for<'a> &'a L: IntoIterator<Item = L::Value>,
      L::Length: Add<B1>,
      Add1<L::Length>: ArrayLength<B> + NonZero + Unsigned,
{
    type Input = L::Value;
    type Output = O;

    fn apply(&mut self, value: Self::Input) -> Self::Output {
        let result = self.bi.iter().skip(1)
            .zip(self.xi.iter())
            .fold(O::from_other(Bx::from_other(self.bi[0]) * Ix::from_other(value)),
                  |accum, (b, x)| accum + O::from_other(Bx::from_other(*b) * Ix::from_other(x)));

        self.xi.push(value);
        result
    }
}

#[cfg(test)]
mod test {
    use crate::{pfdl::Store as DL, U3, si};
    use super::*;

    #[test]
    fn fir_i8_n3() {
        let param = Param::<i8, U3>::from([9, 1, 7, 4]);
        let mut state = State::<i8, i8, DL<i8, U3>, i16, i16>::new(&param, DL::from(0));

        assert_eq!(state.apply(0), 0);
        assert_eq!(state.apply(1), 9);
        assert_eq!(state.apply(0), 1);
        assert_eq!(state.apply(0), 7);
        assert_eq!(state.apply(0), 4);
        assert_eq!(state.apply(0), 0);
        assert_eq!(state.apply(-1), -9);
        assert_eq!(state.apply(1), 8);
        assert_eq!(state.apply(3), 21);
        assert_eq!(state.apply(-7), -57);
        assert_eq!(state.apply(-7), -45);
        assert_eq!(state.apply(10), 46);
    }

    #[test]
    fn fir_fix_base10_n3() {
        type I = si::Micro<i32>;
        type O = si::Nano<i32>;
        type P = si::Milli<i32>;

        let param = Param::<P, U3>::from([P::new(0_456), P::new(-0_137), P::new(0_702), P::new(-1_421)]);
        let mut state = State::<O, P, DL<I, U3>, I, P>::new(&param, DL::from(I::new(0)));

        assert_eq!(state.apply(I::new(0_000)), O::new(0_000));
        assert_eq!(state.apply(I::new(1_000)), O::new(456_000));
        assert_eq!(state.apply(I::new(0_000)), O::new(-137_000));
        assert_eq!(state.apply(I::new(0_000)), O::new(702_000));
        assert_eq!(state.apply(I::new(0_000)), O::new(-1_421_000));
        assert_eq!(state.apply(I::new(0_000)), O::new(0_000));

        assert_eq!(state.apply(I::new(0_123)), O::new(56_088));
        assert_eq!(state.apply(I::new(11_234)), O::new(5_105_853));
        assert_eq!(state.apply(I::new(5_001)), O::new(827_744));
        assert_eq!(state.apply(I::new(-3_120)), O::new(5_603_628));
        assert_eq!(state.apply(I::new(-8_998)), O::new(-16_128_460));
    }
}
