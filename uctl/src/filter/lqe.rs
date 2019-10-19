/*!

## LQE (Kalman) filter

This module implements **Linear Quadratic Estimation** (LQE) filter which also known as Kalman filter.

Filter has four parameters:

- `F` - factor of actual value to previous actual value
- `H` - factor of measured value to actual value
- `Q` - measurement noise
- `R` - environment noise

Filter consists of two stages:

1. Prediction
   - Predict state as _X0 = F * X_
   - Predict covariance _P0 = F^2 * P + Q_
2. Correction
   - Calculate gain as _K = H * P0 / (H^2 * P0 + R)_
   - Calculate covariance as _P = (1 - K * H) * P0_
   - Calculate state as _X = X0 + K * (X - H * X0)_

See also [Kalman filter](https://en.wikipedia.org/wiki/Kalman_filter) article.

 */

use core::{
    marker::PhantomData,
    ops::{Add, Sub, Mul, Div},
};
use typenum::{Sum, Diff, Prod, Quot};
use ufix::Cast;
use crate::Transducer;

/**
LQE filter parameters

- `F` - factor type
- `N` - noise type
- `F2` - square factor type

*/
#[derive(Debug, Clone, Copy)]
pub struct Param<F, N, F2> {
    /// Factor of actual value to previous actual value
    f: F,
    /// Factor of measured value to actual value
    h: F,
    /// Measurement noise
    q: N,
    /// Environment noise
    r: N,

    /// Square f
    f2: F2,
    /// Square h
    h2: F2,
}

impl<F, N, F2> Param<F, N, F2> {
    /**
    Init LQE parameters
     */
    pub fn new<Ff, Fh, Nq, Nr>(f: Ff, h: Fh, q: Nq, r: Nr) -> Self
    where
        F: Copy + Mul<F> + Cast<Ff> + Cast<Fh>,
        N: Cast<Nq> + Cast<Nr>,
        F2: Cast<Prod<F, F>>,
    {
        let f = F::cast(f);
        let h = F::cast(h);

        Self {
            f, h,
            q: N::cast(q),
            r: N::cast(r),

            f2: F2::cast(f * f),
            h2: F2::cast(h * h),
        }
    }
}

/**
LQE filter state

- `O` - output and state type
- `P` - covariance type

*/
#[derive(Debug, Clone, Copy, Default)]
pub struct State<O, P> {
    /// State value
    x: O,
    /// Covariance
    p: P,
}

/**
LQE filter

- `F` - factor type
- `N` - noise type
- `F2` - square factor type
- `I` - input type
- `O` - output and state type
- `P` - covariance type
- `K` - gain type

*/
pub struct Filter<F, N, F2, I, O, P, K>(PhantomData<(F, N, F2, I, O, P, K)>);

impl<F, N, F2, I, O, P, K> Transducer for Filter<F, N, F2, I, O, P, K>
where
    F: Copy + Cast<f64> + Cast<Prod<K, F>> + Mul<I> + Mul<O> + Mul<P> + Sub<F>,
    F2: Copy + Mul<P>,
    O: Copy + Cast<Prod<F, I>> + Cast<Prod<F, O>> + Cast<Prod<K, Diff<O, O>>> + Cast<Sum<O, O>> + Sub<O> + Add<O>,
    P: Copy + Cast<Prod<F2, P>> + Cast<Sum<P, N>> + Cast<Prod<Diff<F, F>, P>> + Add<N>,
    N: Copy + Cast<Prod<F2, P>> + Add<N>,
    Prod<F, P>: Div<Sum<N, N>>,
    K: Copy + Cast<Quot<Prod<F, P>, Sum<N, N>>> + Mul<F> + Mul<Diff<O, O>>,
    Diff<F, F>: Mul<P>,
{
    type Input = I;
    type Output = O;
    type Param = Param<F, N, F2>;
    type State = State<O, P>;

    fn apply(param: &Self::Param, state: &mut Self::State, value: Self::Input) -> Self::Output {
        //
        // Prediction
        //

        // Predict state: X0 = F * X
        let x0 = O::cast(param.f * value);

        // Predict covariance: P0 = F^2 * P + Q
        let p0 = P::cast(P::cast(param.f2 * state.p) + param.q);

        //
        // Correction
        //

        // K = H * P0 / (H^2 * P0 + R)
        let k = K::cast(param.h * p0 / (N::cast(param.h2 * p0) + param.r));

        // P = (1 - K * H) * P0
        state.p = P::cast((F::cast(1.0) - F::cast(k * param.h)) * p0);

        // X = X0 + K * (X - H * X0)
        state.x = O::cast(x0 + O::cast(k * (state.x - O::cast(param.h * x0))));

        state.x
    }
}

#[cfg(test)]
mod test {
    use typenum::*;
    use ufix::{bin::Fix};
    use super::*;

    #[test]
    fn lqe_f32() {
        let param = Param::<f32, f32, f32>::new(0.6, 0.5, 0.2, 0.4);
        let mut state = State::<f32, f32>::default();
        type Filter1 = Filter::<f32, f32, f32, f32, f32, f32, f32>;

        assert_eq!(Filter1::apply(&param, &mut state, 0.123456), 0.0658432);
        assert_eq!(Filter1::apply(&param, &mut state, 1.01246), 0.5400895);
        assert_eq!(Filter1::apply(&param, &mut state, -5.198), -2.4904206);
    }

    #[test]
    fn lqe_fix() {
        type F = Fix<P32, N16>;
        type N = Fix<P32, N16>;
        type I = Fix<P32, N16>;
        type O = Fix<P32, N16>;
        type P = Fix<P32, N16>;
        type K = Fix<P32, N16>;

        let param = Param::<F, N, F>::new(0.6, 0.5, 0.2, 0.4);
        let mut state = State::<O, P>::default();
        type Filter1 = Filter::<F, N, F, I, O, P, K>;

        assert_eq!(Filter1::apply(&param, &mut state, Fix::cast(0.123456)), Fix::cast(0.06584));
        assert_eq!(Filter1::apply(&param, &mut state, Fix::cast(1.01246)), Fix::cast(0.5400895));
        assert_eq!(Filter1::apply(&param, &mut state, Fix::cast(-5.198)), Fix::cast(-2.49045));
    }
}
