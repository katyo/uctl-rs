use super::{Cast, Digits, Exponent, Fix, Mantissa, Radix};

macro_rules! into_impls {
    ($kind:ident: $($type:ident),*) => {
        $(
            impl<R, B, E> From<Fix<R, B, E>> for $type
            where
                R: Radix<B>,
                B: Digits,
                E: Exponent,
                $type: Cast<Mantissa<R, B>>,
            {
                fn from(Fix { bits: value, .. }: Fix<R, B, E>) -> Self {
                    // radix^|exp|
                    let ratio = R::ratio(E::I32.unsigned_abs());
                    // TODO: Add rounding
                    if 0 < E::I32 {
                        into_impls!(@$kind, *, $type, value, ratio)
                    } else {
                        into_impls!(@$kind, /, $type, value, ratio)
                    }
                }
            }
        )*
    };

    (@int, $op: tt, $type: ty, $value: ident, $ratio: ident) => {
        <$type>::cast($value $op $ratio)
    };

    (@float, $op: tt, $type: ty, $value: ident, $ratio: ident) => {
        <$type>::cast($value) $op <$type>::cast($ratio)
    };
}

into_impls!(int: u8, u16, u32, u64, i8, i16, i32, i64);
into_impls!(float: f32, f64);

#[cfg(feature = "i128")]
into_impls!(int: u128, i128);

#[cfg(test)]
mod test {
    use crate::si::Milli;
    use typenum::*;

    #[test]
    fn into_u16() {
        let a = Milli::<P4>::new(9_000);
        assert_eq!(u16::from(a), 9);
    }

    #[test]
    fn into_u16_neg() {
        let a = Milli::<P4>::new(-9_000);
        assert_eq!(u16::from(a), 65527);
    }

    #[test]
    fn into_i16() {
        let a = Milli::<P4>::new(9_000);
        assert_eq!(i16::from(a), 9);
    }

    #[test]
    fn into_i16_neg() {
        let a = Milli::<P4>::new(-9_000);
        assert_eq!(i16::from(a), -9);
    }

    #[test]
    fn into_u32() {
        let a = Milli::<P8>::new(9_000);
        assert_eq!(u32::from(a), 9);
    }

    #[test]
    fn into_i32_neg() {
        let a = Milli::<P8>::new(-9_000);
        assert_eq!(i32::from(a), -9);
    }

    #[test]
    fn into_u64() {
        let a = Milli::<P16>::new(9_000);
        assert_eq!(u64::from(a), 9);
    }

    #[test]
    fn into_i64_neg() {
        let a = Milli::<P16>::new(-9_000);
        assert_eq!(i64::from(a), -9);
    }

    #[test]
    fn into_f32() {
        let a = Milli::<P4>::new(100);
        assert_eq!(f32::from(a), 0.1);

        let a = Milli::<P4>::new(-2_500);
        assert_eq!(f32::from(a), -2.5);
    }

    #[test]
    fn into_f64() {
        let a = Milli::<P4>::new(100);
        assert_eq!(f64::from(a), 0.1);

        let a = Milli::<P4>::new(-2_500);
        assert_eq!(f64::from(a), -2.5);
    }
}
