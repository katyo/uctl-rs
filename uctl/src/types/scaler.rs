use core::{
    ops::{RangeInclusive, Mul, Add, Div, Sub},
    marker::PhantomData,
};
use ufix::{Cast};
use typenum::{Prod, Quot, Sum, Diff};
use crate::{Transducer};

/**

y = (x - x0) / (x1 - x0) * (y1 - y0) + y0;

y = x * (y1 - y0) / (x1 - x0) + y0 - x0 * (y1 - y0) / (x1 - x0);

factor = (y1 - y0) / (x1 - x0);

offset = y0 - x0 * factor;

y = x * factor + offset;

*/
pub struct Param<F, O> {
    factor: F,
    offset: O,
}

impl<F, O> Param<F, O> {
    pub fn new<I>(from: RangeInclusive<I>, to: RangeInclusive<O>) -> Self
    where
        I: Copy + Sub<I>,
        O: Copy + Sub<O> + Sub<Prod<F, I>> + Cast<Diff<O, Prod<F, I>>>,
        F: Copy + Mul<I> + Cast<Quot<Prod<F, I>, Diff<I, I>>>,
        Prod<F, I>: Cast<Diff<O, O>> + Div<Diff<I, I>>,
    {
        let x0 = *from.start();
        let x1 = *from.end();
        let y0 = *to.start();
        let y1 = *to.end();

        let dx = x1 - x0;
        let dy = y1 - y0;

        let factor = F::cast(Prod::<F, I>::cast(dy) / dx);
        let offset = O::cast(y0 - factor * x0);

        Self { factor, offset }
    }
}

pub struct Scaler<I, O, F> {
    val: PhantomData<(I, O, F)>,
}

impl<I, O, F> Transducer for Scaler<I, O, F>
where
    I: Copy,
    O: Copy + Cast<Sum<Prod<F, I>, O>>,
    F: Copy + Mul<I>,
    Prod<F, I>: Add<O>,
{
    type Input = I;
    type Output = O;
    type Param = Param<F, O>;
    type State = ();

    #[inline]
    fn apply(param: &Self::Param, _state: &mut Self::State, value: Self::Input) -> Self::Output {
        O::cast(param.factor * value + param.offset)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn upscale_0to1_0to100() {
        let p = Param::<f32, _>::new(0.0..=1.0, 0.0..=100.0);

        // on bounds
        assert_eq!(Scaler::apply(&p, &mut (), 0.0), 0.0);
        assert_eq!(Scaler::apply(&p, &mut (), 1.0), 100.0);

        // in range
        assert_eq!(Scaler::apply(&p, &mut (), 0.5), 50.0);
        assert_eq!(Scaler::apply(&p, &mut (), 0.65), 65.0);

        // under range
        assert_eq!(Scaler::apply(&p, &mut (), -0.25), -25.0);

        // over range
        assert_eq!(Scaler::apply(&p, &mut (), 1.25), 125.0);
    }

    #[test]
    fn upscale_n1to1_10to20() {
        let p = Param::<f32, _>::new(-1.0..=1.0, 10.0..=20.0);

        // on bounds
        assert_eq!(Scaler::apply(&p, &mut (), -1.0), 10.0);
        assert_eq!(Scaler::apply(&p, &mut (), 1.0), 20.0);

        // in range
        assert_eq!(Scaler::apply(&p, &mut (), 0.0), 15.0);
        assert_eq!(Scaler::apply(&p, &mut (), -0.5), 12.5);

        // under range
        assert_eq!(Scaler::apply(&p, &mut (), -1.25), 8.75);

        // over range
        assert_eq!(Scaler::apply(&p, &mut (), 1.25), 21.25);
    }
}
