/*!

## EMA filter

This module implements **Exponential Moving Average** filter.

EMA is a simple and fast filter which does not require delay line but only single result of previous evaluation as a state.

Filter formula: _y = α * x + (1 - α) * y[-1]_

There are different ways of definition a filter parameters, such as:

1. Using α factor
2. Through number of smoothing steps
3. Through smoothing time
4. As an 1st-order transmission behavior

See also [Exponential moving average](https://en.wikipedia.org/wiki/Moving_average#Exponential_moving_average).

*/

use crate::{Cast, Transducer};
use core::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};
use typenum::{Diff, Prod, Quot, Sum};

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

impl<A> Param<A> {
    /**
    Init EMA parameters using alpha factor

    * `alpha`: The value of α (0..1).

    Usually the alpha factor can be treated as the weight of actual value in result.
    This meaning that when the alpha equals to 1 then no smoothing will be applied.
    The less alpha does more smoothing and vise versa.

    Filter formula: _y = α * x + (1 - α) * y[-1]_
     */
    pub fn from_alpha(alpha: A) -> Self
    where
        A: Copy + Cast<f64> + Sub<A> + Cast<Diff<A, A>>,
    {
        Self {
            alpha,
            one_sub_alpha: A::cast(A::cast(1.0) - alpha),
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
    where
        N: Cast<f64> + Add<N> + Mul<A>,
        A: Copy + Cast<f64> + Sub<A> + Cast<Diff<A, A>> + Cast<Quot<Prod<N, A>, Sum<N, N>>>,
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
    where
        A: Copy + Cast<f64> + Sub<A> + Cast<Diff<A, A>> + Cast<Quot<Sum<P, P>, Sum<T, P>>>,
        T: Cast<P> + Add<P>,
        P: Copy + Add<P>,
        Sum<P, P>: Div<Sum<T, P>>,
    {
        Self::from_alpha(A::cast((period + period) / (time + period)))
    }

    /**
    Init EMA parameters as 1st-order transmission behavior.

    See [PT1](https://de.wikipedia.org/wiki/PT1-Glied).

    _α = 1 / (1 + T/P) = P / (T + P)_

    Filter formula: _y = x * 1 / (1 + T/P) + y[-1] * T / (P + T)_

    See [`Param::from_alpha`](#method.from_alpha).
     */
    pub fn from_pt1<T>(time: T, period: T) -> Self
    where
        A: Copy + Cast<f64> + Sub<A> + Cast<Diff<A, A>> + Cast<Quot<Prod<Sum<T, T>, A>, Sum<T, T>>>,
        T: Copy + Add<T>,
        Sum<T, T>: Mul<A>,
        Prod<Sum<T, T>, A>: Cast<T> + Div<Sum<T, T>>,
    {
        Self::from_alpha(A::cast(
            Prod::<Sum<T, T>, A>::cast(period) / (time + period),
        ))
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

- `O` - filter output value type
*/
#[derive(Debug, Clone, Copy, Default)]
pub struct State<O> {
    /// The last output value
    last_value: O,
}

impl<O> State<O> {
    /**
    Initialize filter state

    - `value`: The initial value
     */
    pub fn new(value: O) -> Self {
        Self { last_value: value }
    }
}

/**
EMA filter

- `A` - filter weights type
- `I` - filter input value type
- `O` - filter output value type
 */
#[derive(Debug)]
pub struct Filter<A, I, O>(PhantomData<(A, I, O)>);

impl<A, I, O> Transducer for Filter<A, I, O>
where
    O: Copy + Add<O> + Cast<Prod<A, I>> + Cast<Prod<A, O>> + Cast<Sum<O, O>>,
    A: Copy + Mul<I> + Mul<O>,
{
    type Input = I;
    type Output = O;
    type Param = Param<A>;
    type State = State<O>;

    fn apply(param: &Self::Param, state: &mut Self::State, value: Self::Input) -> Self::Output {
        // X = alpha * X + (1 - alpha) * X0
        state.last_value =
            O::cast(O::cast(param.alpha * value) + O::cast(param.one_sub_alpha * state.last_value));
        state.last_value
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use typenum::*;
    use ufix::bin::Fix;

    #[test]
    fn from_n_float() {
        let param = Param::<f32>::from_steps(2.0);

        assert_eq!(param.alpha, 0.6666667);
        assert_eq!(param.one_sub_alpha, 0.3333333);

        let mut state = State::<f32>::new(0.0);

        assert_eq!(Filter::apply(&param, &mut state, 1.0), 0.6666667);
        assert_eq!(Filter::apply(&param, &mut state, 1.0), 0.8888889);
    }

    #[test]
    fn from_n_fix_32_64() {
        // We use 64-bit types for multiplication and division to save precision and avoid overflows
        // But also we can use only 32-bit arithmetic with lower precision
        type A = Fix<P32, N18>;
        type V = Fix<P32, N13>;

        let param = Param::<A>::from_steps(Fix::<P31, N18>::cast(2.0));

        assert_eq!(param.alpha, Fix::cast(0.6666667));
        assert_eq!(param.one_sub_alpha, Fix::cast(0.333336));

        let mut state = State::<V>::new(Fix::cast(0.0));

        assert_eq!(
            Filter::apply(&param, &mut state, V::cast(1.0)),
            V::cast(0.6666667)
        );
        assert_eq!(
            Filter::apply(&param, &mut state, V::cast(1.0)),
            V::cast(0.8888889)
        );
    }

    #[test]
    fn from_n_fix_32_only() {
        type A = Fix<P32, N18>;
        type V = Fix<P32, N13>;

        let param = Param::<A>::from_steps(Fix::<P16, N11>::cast(2.0));

        assert_eq!(param.alpha, Fix::cast(0.6666667));
        assert_eq!(param.one_sub_alpha, Fix::cast(0.333336));

        let mut state = State::<V>::new(Fix::cast(0.0));

        assert_eq!(
            Filter::apply(&param, &mut state, V::cast(1.0)),
            V::cast(0.6666667)
        );
        assert_eq!(
            Filter::apply(&param, &mut state, V::cast(1.0)),
            V::cast(0.8888889)
        );
    }
}
