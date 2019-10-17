/*!

## LQE (Kalman) filter

This module implements Linear Quadratic Estimation (LQE) filter which also known as Kalman filter.

See also [Kalman filter](https://en.wikipedia.org/wiki/Kalman_filter) article.

 */

use crate::{FromOther, Transducer, PhantomData, Mul, Add, Sub, Div};

/// LQE filter parameters
#[derive(Debug, Clone, Copy)]
pub struct Param<P> {
    /// Factor of actual value to previous actual value
    pub f: P,
    /// Factor of measured value to actual value
    pub h: P,
    /// Measurement noise
    pub q: P,
    /// Environment noise
    pub r: P,
}

/// LQE filter state
#[derive(Debug, Clone, Copy)]
pub struct State<V> {
    /// State value
    x: V,
    /// Covariance
    p: V,
}

/// LQE filter
pub struct Filter<'p, 's, V, P, Vx, Px> {
    param: &'p Param<P>,
    state: &'s mut State<V>,
    _phantom: PhantomData<(Vx, Px)>,
}

impl<'p, 's, V, P, Vx, Px> Transducer for Filter<'p, 's, V, P, Vx, Px>
where
    V: Copy + FromOther<P> + FromOther<<Px as Mul<Vx>>::Output>,
    P: Copy,
    Vx: FromOther<V> + FromOther<P> + FromOther<<Px as Mul<Vx>>::Output> + Add<Vx, Output = Vx> + Div<Vx>,
    Px: FromOther<P> + Mul<Vx>,
{
    type Input = V;
    type Output = V;

    fn apply(&self, value: Self::Input) -> Self::Output {
        let f = Px::from_other(self.param.f);
        let h = Px::from_other(self.param.h);
        let q = Vx::from_other(self.param.q);
        let r = Vx::from_other(self.param.r);

        let x = Vx::from_other(self.state.x);
        let p = Vx::from_other(self.state.p);

        //
        // Prediction
        //

        // Predict state: X0 = F * X
        let x0 = Vx::from_other(f * x);

        // Predict covariance: P0 = F^2 * P + Q
        let p0 = Vx::from_other(f * Vx::from_other(f * p)) + q;

        //
        // Correction
        //

        // K = H * P0 / (H^2 * P0 + R)
        let k = Vx::from_other(h * p0) / (Vx::from_other(h * Vx::from_other(h * p0)) + r);

        // P = (1 - K * H) * P0
        self.state.p = (Px::from_other(1.0) - k * self.param.h) * p0;

        // X = X0 + K * (X - H * X0)
        self.state.x = x0 + k * (self.state.x - self.param.h * x0);

        self.state.x
    }
}
