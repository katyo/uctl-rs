use crate::{pi, Cast};
use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::Mul,
};
use derive_deref::{Deref, DerefMut};
use typenum::Prod;

/// Angle value in degrees
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut)]
pub struct Deg<D>(pub D);

impl<D> Display for Deg<D>
where
    D: Display,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.fmt(f)?;
        "deg".fmt(f)
    }
}

impl<D, R> From<Rad<R>> for Deg<D>
where
    R: Mul<D>,
    D: Cast<f64> + Cast<Prod<R, D>>,
{
    fn from(Rad(rad): Rad<R>) -> Self {
        Deg(D::cast(rad * D::cast(180.0 / pi::<f64>())))
    }
}

impl<D, H> From<Hpi<H>> for Deg<D>
where
    H: Mul<D>,
    D: Cast<f64> + Cast<Prod<H, D>>,
{
    fn from(Hpi(hpi): Hpi<H>) -> Self {
        Deg(D::cast(hpi * D::cast(90.0)))
    }
}

impl<D, C> From<Cyc<C>> for Deg<D>
where
    C: Mul<D>,
    D: Cast<f64> + Cast<Prod<C, D>>,
{
    fn from(Cyc(cyc): Cyc<C>) -> Self {
        Deg(D::cast(cyc * D::cast(360.0)))
    }
}

/// Angle value in radians
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut)]
pub struct Rad<R>(pub R);

impl<R> Display for Rad<R>
where
    R: Display,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.fmt(f)?;
        "rad".fmt(f)
    }
}

impl<R, D> From<Deg<D>> for Rad<R>
where
    D: Mul<R>,
    R: Cast<f64> + Cast<Prod<D, R>>,
{
    fn from(Deg(deg): Deg<D>) -> Self {
        Rad(R::cast(deg * R::cast(pi::<f64>() / 180.0)))
    }
}

impl<R, H> From<Hpi<H>> for Rad<R>
where
    H: Mul<R>,
    R: Cast<f64> + Cast<Prod<H, R>>,
{
    fn from(Hpi(hpi): Hpi<H>) -> Self {
        Rad(R::cast(hpi * R::cast(pi::<f64>() / 2.0)))
    }
}

impl<R, C> From<Cyc<C>> for Rad<R>
where
    C: Mul<R>,
    R: Cast<f64> + Cast<Prod<C, R>>,
{
    fn from(Cyc(cyc): Cyc<C>) -> Self {
        Rad(R::cast(cyc * R::cast(pi::<f64>() * 2.0)))
    }
}

/// Angle value in ½π units
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut)]
pub struct Hpi<H>(pub H);

impl<H> Display for Hpi<H>
where
    H: Display,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.fmt(f)?;
        "½π".fmt(f)
    }
}

impl<H, D> From<Deg<D>> for Hpi<H>
where
    D: Mul<H>,
    H: Cast<f64> + Cast<Prod<D, H>>,
{
    fn from(Deg(deg): Deg<D>) -> Self {
        Hpi(H::cast(deg * H::cast(1.0 / 90.0)))
    }
}

impl<H, R> From<Rad<R>> for Hpi<H>
where
    R: Mul<H>,
    H: Cast<f64> + Cast<Prod<R, H>>,
{
    fn from(Rad(rad): Rad<R>) -> Self {
        Hpi(H::cast(rad * H::cast(2.0 / pi::<f64>())))
    }
}

impl<H, C> From<Cyc<C>> for Hpi<H>
where
    C: Mul<H>,
    H: Cast<f64> + Cast<Prod<C, H>>,
{
    fn from(Cyc(cyc): Cyc<C>) -> Self {
        Hpi(H::cast(cyc * H::cast(4.0)))
    }
}

/// Angle value in cycles
///
/// 1 cycle == 2π == 360 deg
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut)]
pub struct Cyc<C>(pub C);

impl<C> Display for Cyc<C>
where
    C: Display,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.fmt(f)?;
        "×2π".fmt(f)
    }
}

impl<C, D> From<Deg<D>> for Cyc<C>
where
    D: Mul<C>,
    C: Cast<f64> + Cast<Prod<D, C>>,
{
    fn from(Deg(deg): Deg<D>) -> Self {
        Cyc(C::cast(deg * C::cast(1.0 / 360.0)))
    }
}

impl<C, R> From<Rad<R>> for Cyc<C>
where
    R: Mul<C>,
    C: Cast<f64> + Cast<Prod<R, C>>,
{
    fn from(Rad(rad): Rad<R>) -> Self {
        Cyc(C::cast(rad * C::cast(1.0 / (2.0 * pi::<f64>()))))
    }
}

impl<C, H> From<Hpi<H>> for Cyc<C>
where
    H: Mul<C>,
    C: Cast<f64> + Cast<Prod<H, C>>,
{
    fn from(Hpi(hpi): Hpi<H>) -> Self {
        Cyc(C::cast(hpi * C::cast(1.0 / 4.0)))
    }
}
