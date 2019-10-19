/*!

## EMA filter

This module implements Exponential Moving Average filter.

EMA is a simple and fast filter which does not require delay line but only single result of previous evaluation as a state.

See also [Exponential moving average](https://en.wikipedia.org/wiki/Moving_average#Exponential_moving_average).

*/

use core::{
  marker::PhantomData,
  ops::{Add, Sub, Mul, Div},
};
use typenum::{Prod, Quot, Sum};
use ufix::Cast;
use crate::Transducer;

/**
EMA filter parameters

 - `A` - filter weights type
*/
#[derive(Debug, Clone, Copy)]
pub struct Param<A> {
    /// The value of alpha parameter
    alpha: A,
    /// The value of 1-alpha parameter
    one_sub_alpha: A,
}

impl<A> Param<A>
where A: Copy + Cast<f64> + Sub<Output = A>,
{
    /**
    Init EMA parameters using alpha factor

    * `alpha`: The value of α (0..1).

    Usually the alpha factor can be treated as the weight of actual value in result.
    This meaning that when the alpha equals to 1 then no smoothing will be applied.
    The less alpha does more smoothing and vise versa.

    Filter formula: _y = α * x + (1 - α) * y[-1]_
     */
    pub fn from_alpha(alpha: A) -> Self {
        Self {
            alpha,
            one_sub_alpha: A::cast(1.0) - alpha,
        }
    }

    /**
    Init EMA parameters using N factor

    * `n`: The value of N factor [1.0 ..]

    Usually the N factor can be treated as the number of steps for smoothing.
    This means that when the N equals to 1 then no smoothing will be applied.
    The more N does more smoothing and vise versa.

    _α = 2 / (n + 1)_

    Filter formula: _y = x * 2 / (n + 1) + y[-1] * (n - 1) / (n + 1)_

    See [`Param::from_alpha`](#method.from_alpha).
    */
    pub fn from_steps<N>(n: N) -> Self
    where N: Cast<f64> + Add<N> + Mul<A>,
          A: Cast<Quot<Prod<N, A>, Sum<N, N>>>,
          Prod<N, A>: Cast<f64> + Div<Sum<N, N>>,
    {
        // α = 2 / (n + 1)
        Self::from_alpha(A::cast(<Prod<N, A>>::cast(2.0) / (n + N::cast(1.0))))
    }

    /**
    Init EMA parameters using time factor

    * `time`: The smooth time value
    * `period`: The sampling time (or control step period)

    _α = 2 / (T/P + 1) = 2 * P / (T + P)_

    _time = period => α = 1_

    _time > period => α < 1_

    Filter formula: _y = x * 2 / (T/P + 1) + y[-1] * (T/P - 1) / (T/P + 1)_

    See [`Param::from_alpha`](#method.from_alpha).
     */
    pub fn from_time<T, P>(time: T, period: P) -> Self
    where T: Cast<P> + Add<Output = T>,
          P: Copy + Add<P>,
          Sum<P, P>: Div<T, Output = A>,
    {
        Self::from_alpha((period + period) / (time + T::cast(period)))
    }

    /**
    Init EMA parameters as 1st-order transmission behavior.

    See [PT1](https://de.wikipedia.org/wiki/PT1-Glied).

    _α = 1 / (1 + T/P) = P / (T + P)_

    Filter formula: _y = x * 1 / (1 + T/P) + y[-1] * T / (P + T)_

    See [`Param::from_alpha`](#method.from_alpha).
     */
    pub fn from_pt1<T, P>(time: T, period: P) -> Self
    where T: Cast<P> + Add<Output = T>,
          P: Copy + Div<T, Output = A>,
    {
        Self::from_alpha(period / (time + T::cast(period)))
    }

    /// Adjust parameters gain
    pub fn with_gain<G>(self, gain: G) -> Param<Prod<A, G>>
    where
        G: Copy,
        A: Mul<G>,
    {
        Param {
            alpha: self.alpha * gain,
            one_sub_alpha: self.one_sub_alpha * gain,
        }
    }

    /// Change parameters type
    pub fn to<B>(self) -> Param<B>
    where
        B: Cast<A>,
    {
        Param {
            alpha: B::cast(self.alpha),
            one_sub_alpha: B::cast(self.one_sub_alpha),
        }
    }
}

/**
EMA filter state

- `V` - filter input/output value type
*/
#[derive(Debug, Clone, Copy, Default)]
pub struct State<V> {
    /// The last output value
    last_value: V,
}

impl<V> State<V> {
    /**
    Initialize filter state

    - `value`: The initial value
     */
    pub fn new(value: V) -> Self {
        Self {
            last_value: value,
        }
    }
}

/**
EMA filter

- `A` - filter weights type
- `V` - filter input/output value type
 */
#[derive(Debug)]
pub struct Filter<A, V>(PhantomData<(A, V)>);

impl<A, V> Transducer for Filter<A, V>
where V: Copy + Cast<<A as Mul<V>>::Output> + Add<V, Output = V>,
      A: Copy + Mul<V>,
{
    type Input = V;
    type Output = V;
    type Param = Param<A>;
    type State = State<V>;

    fn apply(param: &Self::Param, state: &mut Self::State, value: Self::Input) -> Self::Output {
        // X = alpha * X + (1 - alpha) * X0
        state.last_value = V::cast(param.alpha * value) +
            V::cast(param.one_sub_alpha * state.last_value);
        state.last_value
    }
}

#[cfg(test)]
mod test {
    use typenum::*;
    use ufix::bin::Fix;
    use super::*;

    #[test]
    fn ewma_n_float() {
        let param = Param::<f32>::from_steps(2.0);

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

        let param = Param::<A>::from_steps(Fix::<P32, N18>::cast(2.0));

        assert_eq!(param.alpha, Fix::cast(0.6666667));
        assert_eq!(param.one_sub_alpha, Fix::cast(0.333336));

        let mut state = State::<V>::new(Fix::cast(0.0));

        assert_eq!(Filter::apply(&param, &mut state, V::cast(1.0)), V::cast(0.6666667));
        assert_eq!(Filter::apply(&param, &mut state, V::cast(1.0)), V::cast(0.8888889));
    }

    #[test]
    fn ewma_n_fix_32_only() {
        type A = Fix<P32, N18>;
        type V = Fix<P32, N13>;

        let param = Param::<A>::from_steps(Fix::<P16, N11>::cast(2.0));

        assert_eq!(param.alpha, Fix::cast(0.6666667));
        assert_eq!(param.one_sub_alpha, Fix::cast(0.333336));

        let mut state = State::<V>::new(Fix::cast(0.0));

        assert_eq!(Filter::apply(&param, &mut state, V::cast(1.0)), V::cast(0.6666667));
        assert_eq!(Filter::apply(&param, &mut state, V::cast(1.0)), V::cast(0.8888889));
    }
}
