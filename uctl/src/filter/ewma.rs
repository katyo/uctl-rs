/*!

## EWMA filter

This module implements Exponentially Weighted Moving Average filter.

See wiki article [EWMA](https://en.wikipedia.org/wiki/Moving_average#Exponential_moving_average).

*/

use crate::{Add, Sub, Mul, Div, FromOther, Transducer, PhantomData};
use typenum::{Prod, Quot, Sum};

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
    pub fn from_n<N>(n: N) -> Self
    where N: FromOther<f64> + Add<N> + Mul<A>,
          A: FromOther<Quot<Prod<N, A>, Sum<N, N>>>,
          Prod<N, A>: FromOther<f64> + Div<Sum<N, N>>,
    {
        Self::from_alpha(A::from_other(<Prod<N, A>>::from_other(2.0) / (n + N::from_other(1.0))))
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
#[derive(Debug, Clone, Copy)]
pub struct State<V> {
    /// The last output value
    last_value: V,
}

impl<V> State<V> {
    /// Initialize filter state
    ///
    /// * `value`: The initial value
    pub fn new(value: V) -> Self {
        Self {
            last_value: value,
        }
    }
}

/// EWMA filter
#[derive(Debug)]
pub struct Filter<A, V>(PhantomData<(A, V)>);

impl<A, V> Transducer for Filter<A, V>
where V: Copy + FromOther<<A as Mul<V>>::Output> + Add<V, Output = V>,
      A: Copy + Mul<V>,
{
    type Input = V;
    type Output = V;
    type Param = Param<A>;
    type State = State<V>;

    fn apply(param: &Self::Param, state: &mut Self::State, value: Self::Input) -> Self::Output {
        // X = alpha * X + (1 - alpha) * X0
        state.last_value = V::from_other(param.alpha * value) +
            V::from_other(param.one_sub_alpha * state.last_value);
        state.last_value
    }
}

#[cfg(test)]
mod test {
    use crate::{bin::{Fix}};
    use typenum::*;
    use super::*;

    #[test]
    fn ewma_n_float() {
        let param = Param::<f32>::from_n(2.0);

        assert_eq!(param.alpha, 0.6666667);
        assert_eq!(param.one_sub_alpha, 0.3333333);

        let mut state = State::<f32>::new(0.0);

        assert_eq!(Filter::apply(&param, &mut state, 1.0), 0.6666667);
        assert_eq!(Filter::apply(&param, &mut state, 1.0), 0.8888889);
    }

    #[test]
    fn ewma_n_fix_32_64() {
        // We use 64-bit types for multiplication and division to save precision and avoid overflows
        // But also we can use only 32-bit arithmetic with lower precision
        type A = Fix<P32, N18>;
        type V = Fix<P32, N13>;

        let param = Param::<A>::from_n(Fix::<P32, N18>::from_other(2.0));

        assert_eq!(param.alpha, Fix::from_other(0.6666667));
        assert_eq!(param.one_sub_alpha, Fix::from_other(0.333336));

        let mut state = State::<V>::new(Fix::from_other(0.0));

        assert_eq!(Filter::apply(&param, &mut state, V::from_other(1.0)), V::from_other(0.6666667));
        assert_eq!(Filter::apply(&param, &mut state, V::from_other(1.0)), V::from_other(0.8888889));
    }

    #[test]
    fn ewma_n_fix_32_only() {
        type A = Fix<P32, N18>;
        type V = Fix<P32, N13>;

        let param = Param::<A>::from_n(Fix::<P16, N11>::from_other(2.0));

        assert_eq!(param.alpha, Fix::from_other(0.6666667));
        assert_eq!(param.one_sub_alpha, Fix::from_other(0.333336));

        let mut state = State::<V>::new(Fix::from_other(0.0));

        assert_eq!(Filter::apply(&param, &mut state, V::from_other(1.0)), V::from_other(0.6666667));
        assert_eq!(Filter::apply(&param, &mut state, V::from_other(1.0)), V::from_other(0.8888889));
    }
}
