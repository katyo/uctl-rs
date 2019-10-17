/*!

## EWMA filter

This module implements Exponentially Weighted Moving Average filter.

See wiki article [EWMA](https://en.wikipedia.org/wiki/Moving_average#Exponential_moving_average).

*/

use crate::{Add, Sub, Mul, Div, FromOther, Filter, Transducer, PhantomData};

/// EWMA filter parameters
///
/// - `A` - filter weights type
#[derive(Debug, Clone, Copy)]
pub struct Param<A> {
    /// The value of alpha parameter
    alpha: A,
    /// The value of 1-alpha parameter
    one_sub_alpha: A,
}

impl<A> Param<A>
where A: Copy + FromOther<f64> + Sub<Output = A>,
{
    /// Init EWMA parameters using alpha factor
    ///
    /// Usually the alpha factor can be treated as the weight of actual value in result.
    /// This meaning that when the alpha equals to 1 then no smoothing will be applied.
    /// The less alpha does more smoothing and vise versa.
    pub fn from_alpha(alpha: A) -> Self {
        Self {
            alpha,
            one_sub_alpha: A::from_other(1.0) - alpha,
        }
    }
}

impl<A> Param<A>
where A: Copy + FromOther<f64> + Sub<Output = A>,
{
    /// Init EWMA parameters using N factor
    ///
    /// * `n`: The value of N factor [1.0 ..]
    ///
    /// Usually the N factor can be treated as the number of steps for smoothing.
    /// This meaning that when the N equals to 1 then no smoothing will be applied.
    /// The more N does more smoothing and vise versa.
    pub fn from_n<M, N>(n: N) -> Self
    where M: FromOther<f64> + Div<<N as Add>::Output>,
          N: FromOther<f64> + Add<N>,
          A: FromOther<<M as Div<<N as Add>::Output>>::Output>,
    {
        Self::from_alpha(A::from_other(M::from_other(2.0) / (n + N::from_other(1.0))))
    }
}

impl<A> Param<A>
where A: Copy + FromOther<f64> + Sub<Output = A>,
{
    /// Init EWMA parameters using time factor
    ///
    /// * `time`: The smooth time value
    /// * `period`: The sampling time (control step period)
    ///
    /// alpha = 2 / (time / period + 1) or
    /// alpha = (2 * period) / (time + period)
    /// time = period => alpha = 1
    /// time > period => alpha < 1
    pub fn from_time<T, P>(time: T, period: P) -> Self
    where T: FromOther<P> + Add<Output = T>,
          P: Copy + Add<P>,
          <P as Add>::Output: Div<T, Output = A>,
    {
        Self::from_alpha((period + period) / (time + T::from_other(period)))
    }
}

/// EWMA filter state
///
/// - `V` - filter input/output value type
/// - `A` - filter weights type
/// - `Vx`, `Ax` - internal types for multiplication
#[derive(Debug)]
pub struct State<'s, V, A, Vx, Ax> {
    /// The filter parameters
    param: &'s Param<A>,
    /// The last output value
    last_value: V,
    /// Phantom data
    _phantom: PhantomData<(Vx, Ax)>,
}

impl<'s, V, A, Vx, Ax> State<'s, V, A, Vx, Ax>
where Vx: FromOther<V>,
{
    /// Initialize filter state
    ///
    /// * `value`: The initial value
    pub fn new(param: &'s Param<A>, value: V) -> Self {
        Self {
            param,
            last_value: value,
            _phantom: PhantomData,
        }
    }
}

impl<'s, V, A, Vx, Ax> Filter for State<'s, V, A, Vx, Ax>
where V: Copy + FromOther<<Ax as Mul<Vx>>::Output> + Add<V, Output = V>,
      A: Copy,
      Ax: FromOther<A> + Mul<Vx>,
      Vx: FromOther<V> + Add<Vx, Output = Vx>,
{
    type Input = V;
    type Output = V;

    fn apply(&mut self, value: Self::Input) -> Self::Output {
        self.last_value = V::from_other(Ax::from_other(self.param.alpha) * Vx::from_other(value)) +
            V::from_other(Ax::from_other(self.param.one_sub_alpha) * Vx::from_other(self.last_value));
        self.last_value
    }
}

#[cfg(test)]
mod test {
    use crate::{bin::{IFix32, IFix64}, *};
    use super::*;

    #[test]
    fn ewma_n_float() {
        let param = Param::from_n::<f32, f32>(2.0);

        assert_eq!(param.alpha, 0.6666667);
        assert_eq!(param.one_sub_alpha, 0.3333333);

        let mut state = State::<f32, f32, f32, f32>::new(&param, 0.0);

        assert_eq!(state.apply(1.0), 0.6666667);
        assert_eq!(state.apply(1.0), 0.8888889);
    }

    #[test]
    fn ewma_n_fix_32_64() {
        // We use 64-bit types for multiplication and division to save precision and avoid overflows
        // But also we can use only 32-bit arithmetic with lower precision
        type A = IFix32::<N18>;
        type V = IFix32::<N13>;

        let param = Param::<A>::from_n::<IFix64<N38>, IFix64<N18>>(Fix::from_other(2.0));

        assert_eq!(param.alpha, Fix::from_other(0.6666667));
        assert_eq!(param.one_sub_alpha, Fix::from_other(0.333336));

        let mut state = State::<V, A, IFix64<N16>, IFix64<N18>>::new(&param, Fix::from_other(0.0));

        assert_eq!(state.apply(V::from_other(1.0)), V::from_other(0.6666667));
        assert_eq!(state.apply(V::from_other(1.0)), V::from_other(0.8888889));
    }

    #[test]
    fn ewma_n_fix_32_only() {
        type A = IFix32::<N18>;
        type V = IFix32::<N13>;

        let param = Param::<A>::from_n::<IFix32<N28>, IFix32<N11>>(Fix::from_other(2.0));

        assert_eq!(param.alpha, Fix::from_other(0.6666667));
        assert_eq!(param.one_sub_alpha, Fix::from_other(0.333336));

        let mut state = State::<V, A, IFix32<N13>, IFix32<N15>>::new(&param, Fix::from_other(0.0));

        assert_eq!(state.apply(V::from_other(1.0)), V::from_other(0.6666667));
        assert_eq!(state.apply(V::from_other(1.0)), V::from_other(0.8888889));
    }
}
