use crate::Cast;
use core::f64::consts::{E, PI, SQRT_2};

/// The π constant
pub fn pi<T>() -> T
where
    T: Cast<f64>,
{
    T::cast(PI)
}

/// The τ constant (2π)
pub fn tau<T>() -> T
where
    T: Cast<f64>,
{
    T::cast(PI * 2.0)
}

/// The e constant
pub fn e<T>() -> T
where
    T: Cast<f64>,
{
    T::cast(E)
}

/// The φ constant (golden ratio)
pub fn phi<T>() -> T
where
    T: Cast<f64>,
{
    T::cast(1.618_033_988_749_895)
}

/// The √2 constant
pub fn sqrt2<T>() -> T
where
    T: Cast<f64>,
{
    T::cast(SQRT_2)
}

/// The √3 constant
pub fn sqrt3<T>() -> T
where
    T: Cast<f64>,
{
    T::cast(1.732_050_807_568_877_2)
}

#[cfg(test)]
mod test {
    use super::*;
    use typenum::*;
    use ufix::Fix;

    #[test]
    fn pi_float_32() {
        assert_eq!(pi::<f32>(), core::f32::consts::PI);
    }

    #[test]
    fn pi_float_64() {
        assert_eq!(pi::<f64>(), core::f64::consts::PI);
    }

    #[test]
    fn pi_fix_p2_15n13() {
        assert_eq!(f64::cast(pi::<Fix<P2, P15, N13>>()), 3.1414794921875);
    }

    #[test]
    fn pi_fix_p2_32n16() {
        assert_eq!(f64::cast(pi::<Fix<P2, P32, N16>>()), 3.1415863037109375);
    }

    #[test]
    fn pi_fix_p2_32n29() {
        assert_eq!(f64::cast(pi::<Fix<P2, P32, N29>>()), 3.1415926534682512);
    }

    #[test]
    fn pi_fix_p2_63n60() {
        assert_eq!(f64::cast(pi::<Fix<P2, P63, N60>>()), core::f64::consts::PI);
    }

    #[cfg(feature = "i128")]
    #[test]
    fn pi_fix_p2_128n124() {
        assert_eq!(
            f64::cast(pi::<Fix<P2, P128, N124>>()),
            core::f64::consts::PI
        );
    }

    #[test]
    fn pi_fix_u2_15n13() {
        assert_eq!(f64::cast(pi::<Fix<U2, P15, N13>>()), 3.1414794921875);
    }

    #[test]
    fn pi_fix_u2_32n30() {
        assert_eq!(f64::cast(pi::<Fix<U2, P32, N30>>()), 3.1415926534682512);
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn pi_fix_p10_12n10() {
        assert_eq!(f64::cast(pi::<Fix<P10, P12, N10>>()), 3.1415926535);
    }

    #[test]
    fn pi_fix_p10_18n16() {
        assert_eq!(f64::cast(pi::<Fix<P10, P18, N16>>()), core::f64::consts::PI);
    }

    #[test]
    fn pi_fix_u10_16n15() {
        assert_eq!(f64::cast(pi::<Fix<U10, P16, N15>>()), core::f64::consts::PI);
    }
}
